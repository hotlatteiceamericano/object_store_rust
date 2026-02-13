S3-Like Object Store

# Object Store
## SegmentStore
It stores objects smaller than 30KB. Each physical files has its overhead and it is wasteful to create new file for the storing of small objects. Hence they are stored together within the same file called Segments, until the size of segment reaches a defined threshold.

Since SegmentStore stores the active segment in it as a field, the instantiation is more expensive than the other type of store - StandaloneStore. Hence SegmentStore is instantiated once during the app starting and being save in the axum state.

The segment component is coming from [rust_segment](https://github.com/hotlatteiceamericano/segment_rust) crate. It create a new Blob struct which implements the Storable trait required by the rust_segment crate and store binary in the BLob struct.

### save()
It finds an active segment whose size is smaller than the defined threshold, and append the binary to this active segment.
When a segment is full, it rotates to a new segment.

It returns the metadata for this object.

### open()
As the active segment is found during the initialization, it can call the active segment's read method directly to read the object.

## StandaloneStore
As opposed to SegmentStore, it stores larger objects in standalone file. As the object becomes larger, metadata for physical files become less wasteful compared to smaller objects.

It returns the metadata for this object.

### save()
As oppose to SegmentStore who needs to store an active segment in it as a field, StandaloneStore is a state-less struct which perform on-and-off action. Hence axum handler instantiate a new StandaloneStore instance in every save request.

### open()


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

## Save()
Save to database using sled. Returning Result<>.

## Read()
*Static* method to return the metadata given bucket, prefix and filename.

# HTTP Layer
## Save
Iimplemented with HTTP PUT method. It asks for for bucket, prefix, filename and the object binary as arguments.

Depends on the object size, it uses different types of object store to store the object.

## Read
Implemented with HTTP GET method. It asks for bucket, prefix and filenames to find the metadata of an object.

It then finds the object store from the metadata, and call the matching object store, SegmentStore or StandaloneStore, to fetch the object.

## List
HTTP GET, ask for bucket, an optional prefix as arguments. Return a list of filename.

It finds a list of metadata by the given bucket and prefix. Then return those metadata.

## Write Path
(better with a diagram to luustrate)
main > http handlers > decide standalone or segement store based on file size > object store

# Future Phases
* Support range read
* Support versioning
* Scale:
  * The first one would be sharding to many different servers. Only if two requests routed to the same shard will be locked.
  * The second one would be *buffering* the write request in RAM. And flush the write request to hard disk when necessary.


# TODOs
1. [x] implement standalone store's read
1. [x] refactor standalone with async
1. [x] implement Metadata::save
1. [x] implement Metadata::read
1. [x] then implement object's read handler, so that it can locate the object with bucket, prefix and filename
1. [x] read again the object_handler::get_object function => 
       AI uses Option::context to convert all the Options to Results
1. [] write tests for Metadata::read
1. [] write tests for ObjectHandler::get_object
1. [] refactor object store's save with async
1. [] 
