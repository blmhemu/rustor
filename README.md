# rustor
A simple file server to learn rust and warp. 

**Note: This is a learner's project and is not remotely usable. Also, this can be implemented in a simpler way using `warp::fs::dir`, but where's the fun in that.**

## Build and run
* **Required:** Edit the `BASE_FOLDER` in `handlers.rs` to point to the folder you want to serve. (I know this is suboptimal).
* Change the IP (say from 0.0.0.0 to 127.0.0.1) and Port (say 3030:8080) per your needs. (Again, suboptimal and should ideally be read from a config file)
* Run `cargo run`

## API
* WebUI can be accessed at http://IP:PORT/web/ls
* Fetch the list of directories at `my/dir` -> `GET http://IP:PORT/api/ls?path=my%2Fdir`
* Download the file at `my/dir/file.ext` -> `GET http://IP:PORT/api/download?path=my%2Fdir%2Ffile.ext`
* Delete the file/folder at `my/dir/rec` -> `GET http://IP:PORT/api/rm?path=my%2Fdir%2Frec`

## Dependencies
* warp, hyper, tokio
* [sailfish](https://github.com/Kogia-sima/sailfish) for web templating.

## Usable Open Source Alternatives
* [miniserve](https://github.com/svenstaro/miniserve)
* [filebrowser](https://filebrowser.org/)
* [filestash](https://www.filestash.app/)
