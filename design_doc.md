S3-Like Object Store

# Object Store
## Requirements
* small file smaller than or equal to 10KB will be packed together with other smaller file in a single segment, until the segment exceed 100KB.
* larger files bigger than 10KB will be stored in a standalong file
* Object get assigned with an UUID


# Metadata Store
* use sled, Rust's embedded key-value store, suitable for prefix search
* store the physical location of the object
* schema:
  * ObjectStore:
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

# Second Phase
* Large file, presumably larger than 10MB, are supposed to be used streaming for uploading and dowloading instead of all being loaded into memory at once.

