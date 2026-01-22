S3-Like Object Store

# Object Store
## Requirements
* Initially one file per object, then extend to support small file by packing different small objects to the same file
* Hash the content, and use the first two characters as the first directory; third to fourth characters as the second directory as storing path
* Object get assigned with an UUID
* small object smaller than 100KB gets packed with other small files

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
      * segment_id: PathBuf
      * offset: u64
      * length: u64
    * External:
      * file_path: PathBuf

# HTTP Layer
* HTTP PUT: given object, bucket, prefix and file_name, upload object
* HTTP GET: given bucket, prefix and file_name, download object
* HTTP GET: given bucket, optional prefix, return list of file names


