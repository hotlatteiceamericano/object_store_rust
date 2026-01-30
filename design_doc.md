S3-Like Object Store

# Object Store
## SegmentStore
This type of object store handle the object storing for smaller file smaller than 30KB. As we are storing the files using files and each file has its own inode for its metadata, storing smaller files in individual files will consume too much of disk spaces. Hence they are stored together within the same file until the file size reaches the to-be-decided threshold.


## StandaloneStore
* This type of object store handle larger files. 
* small file smaller than or equal to 10KB will be packed together with other smaller file in a single segment, until the segment exceed 100KB.
* larger files bigger than 10KB will be stored in a standalong file
* Object get assigned with an UUID


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

# HTTP Layer
* HTTP PUT: given object, bucket, prefix and file_name, upload object
* HTTP GET: given bucket, prefix and file_name, download object
* HTTP GET: given bucket, optional prefix, return list of file names

## Write Path
main > http handlers > decide standalone or segement store based on file size > object store

# Future Phases
* Support range read.

# TODOs
1. [x] implement standalone store's read
1. [x] refactor standalone with async
1. [x] implement Metadata::save
1. [x] implement Metadata::read
1. [x] then implement object's read handler, so that it can locate the object with bucket, prefix and filename
1. [] read again the object_handler::get_object function
1. [] write tests for Metadata::read
1. [] write tests for ObjectHandler::get_object
1. [] refactor object store's save with async
1. [] 
