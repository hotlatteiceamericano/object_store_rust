S3-Like Object Store

# Object Store
## SegmentStore
It stores objects smaller than 1KB. Each physical files has its overhead and it is wasteful to create new file for the storing of small objects. Hence they are appended and stored together within the same file called Segments, until the size of segment reaches a defined threshold.

Since SegmentStore stores the active segment in it as a field, the instantiation is more expensive than the other type of store - StandaloneStore. Hence SegmentStore is instantiated once during the app starting and being save in the axum state.

The segment component is coming from [rust_segment](https://github.com/hotlatteiceamericano/segment_rust) crate. It create a new Blob struct which implements the Storable trait required by the rust_segment crate and store binary in the BLob struct.

### save()
It finds an active segment whose size is smaller than the defined threshold, and append the binary to this active segment.
When a segment is full, it rotates to a new segment.

It returns the metadata for this object.

### open()
SegmentStore finds the active segment file from its field, and call the active segmnet's find method to read the binary, and output to the caller as stream.

## StandaloneStore
As opposed to SegmentStore, it stores larger objects in standalone file. As the object becomes larger, metadata for physical files become less wasteful compared to smaller objects.

It returns the metadata for this object.

### save()
As oppose to SegmentStore who needs to store an active segment in it as a field, StandaloneStore is a state-less struct which perform on-and-off action. Hence axum handler instantiate a new StandaloneStore instance in every save request.

### open(path: Path)
Read the binary from the given path and output it as stream.

# Metadata Store
* use sled, Rust's embedded key-value store, suitable for prefix search
* store the physical location of the object
* schema:
  * Metadata:
    * object_id: u64
    * checksum: 
    * version:
    * storage_type: StorageType

  * StorageType
    * Packed
      * segment_file_path: PathBuf
      * offset: u64
      * length: u64
    * Standalone:
      * file_path: PathBuf

## save()
Save to database using sled. Returning Result<>.

## read()
*Static* method to return the metadata given bucket, prefix and filename.

## list()
Takes mandatory bucket, optional prefix and optional filename, return list of metadata.

# HTTP Layer
## save
Implemented with HTTP PUT method. It asks for for bucket, prefix, filename and the object binary as arguments.

Depends on the object size, it uses different types of object store to store the object.

## read
Implemented with HTTP GET method. It asks for bucket, prefix and filenames to find the metadata of an object.

It then finds the object store from the metadata, and call the matching object store, SegmentStore or StandaloneStore, to fetch the object.

## list
HTTP GET, ask for bucket, an optional prefix as arguments. Return a list of filename.

It finds a list of metadata by the given bucket and prefix. Then return those metadata.

## Write Path
(better with a diagram to illustrate)
main > http handlers > decide standalone or segement store based on file size > object store

# Future Extensions
## Features:
* Support range read
* Support versioning

## Scaling:
* metadata migrate from embedded sled to a relational/nosql database
* decouple the storage layer from storage layer. when API layer receives write request, it founds an available server to write the data

## Deployment
It uses GitHub's workflow to build the docker image, and push to GitHub's image registry: GHCR. It then uses the GitHub action provided by fly "flyctl" to run the deployment command, provide with the registry's location.


