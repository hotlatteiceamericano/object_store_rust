S3-Like Object Store

# Object Store
## SegmentStore
This type of object store handle the object storing for smaller file smaller than 30KB. As we are storing the files using files and each file has its own inode for its metadata, storing smaller files in individual files will consume too much of disk spaces. Hence they are stored together within the same file until the file size reaches the to-be-decided threshold.

### save()
It finds an active segment and append the binary to this active segment.

In order for SegmentStore to improve save/write performance, SegmentStore will search for an active segment under the data/segment path and store in it as a field. As oppose to what StandlaoneStore does which will be discussed shortly.

SegmentStore also in responsible for segment's rotation and manage the active segment file, that the segment is not full yet. In order for the segment file to be locked and shared between different requests/handlers, `Arc<Mutex>>` will be applied to SegmentStore.

Having Arc<> inside the SegmentStore is how sled::Db does is because, the sled::Db is a public library meant to be shared. For something only being used locally within a project, it is not necessarily to hide this implementation too deep. As a result, we will wrap SegmentStore with Arc<> during the state.


To scale further, two mechanics can be implemented to remove the locking bottleneck.

The first one would be sharding to many different servers. Only if two requests routed to the same shard will be locked.

The second one would be *buffering* the write request in RAM. And flush the write request to hard disk when necessary.

## StandaloneStore
* This type of object store handle larger files. 
* Files smaller than or equal to 10KB will be packed together with other smaller file in a single segment, until the segment exceed 100KB.
* larger files bigger than 10KB will be stored in a standalong file

### open()

### save()
It creates a new file with a incremental number as the standalone filename and store the binary in it. As oppose to SegmentStore who needs to store an active segment in it as a field, StandaloneStore is a state-less struct which perform on-and-off action. And the axum handler spawn a new StandaloneStore instance in every save request.


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
HTTP PUT, ask for bucket, prefix, filename and the object binary as arguments.

It first saves the object using a type of store depends on the file size. Store will then return the object location. Object location will be store in the metadata along with bucket, prefix and filename.

## Read
HTTP GET, ask for bucket, prefix and filename as arguments.

It first finds the metadata by bucket, prefix and filename. Then gets the object location from the metadata. Read the object with that location and return.

## List
HTTP GET, ask for bucket, an optional prefix as arguments. Return a list of filename.

It finds a list of metadata by the given bucket and prefix. Then return those metadata.

## Write Path
(better with a diagram to luustrate)
main > http handlers > decide standalone or segement store based on file size > object store

# Future Phases
* Support range read.

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
