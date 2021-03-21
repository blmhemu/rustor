# rustor
A simple file server to learn rust and warp. 

**Note: This is a learner's project and is not remotely usable.** 

**This can probably be implemented in a simpler way using `warp::fs::dir`, but where's the fun in that.**

## Build and run
* **Required:** Edit the `BASE_FOLDER` in `handlers.rs` to point to the folder you want to serve. (I know this is suboptimal).
* Change the IP (say from 0.0.0.0 to 127.0.0.1) and Port (say 3030 to 8080) per your needs. (Again, suboptimal and should ideally be read from a config file)
* Run `cargo run`
* Experimental UI can be accessed by running `pnpm dev` or `npm dev` in `sapper` folder (Backend must be on localhost:3030)
* Even more experimental UI can be accessed by running `pnpm dev` or `npm dev` in `svelteui` folder (Backend must be on localhost:3030)

## API
* WebUI can be accessed at http://IP:PORT/web/ls
* Fetch the list of directories at `my/dir` -> `GET http://IP:PORT/api/ls?path=my%2Fdir`
* Download the file at `my/dir/file.ext` -> `GET http://IP:PORT/api/download?path=my%2Fdir%2Ffile.ext`
* Delete the file/folder at `my/dir/rec` -> `GET http://IP:PORT/api/rm?path=my%2Fdir%2Frec`

## Capabilities / Features
- [x] File and path sanitization to prevent directory attacks.
- [x] View and traverse folders
- [x] Download files
- [x] Create folders
- [x] Delete folders and files
- [x] Upload files (tested with file size of ~ 12GB on localhost)
- [ ] Multi tenant and auth
- [ ] Rename dir / file
- [ ] Move file / dir


## Todos
- [ ] Code Cleanup, Import Optimization
- [ ] Logging

## Dependencies
* warp, hyper, tokio for the basic web server / concurrency stuff.
* [sailfish](https://github.com/Kogia-sima/sailfish) for web templating.
* Other helper libs.

## Thanks
* [miniserve](https://github.com/svenstaro/miniserve) for parts of code mainly HTML / CSS / JS

## Usable Open Source Alternatives
* [miniserve](https://github.com/svenstaro/miniserve)
* [filebrowser](https://filebrowser.org/)
* [filestash](https://www.filestash.app/)
