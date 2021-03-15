# rustor
A simple file server to learn rust and warp. 

Note: This can be done in a much simpler way using `warp::fs::dir`

## Build and run
* **Required:** Edit the `BASE_FOLDER` to point to the folder you want to serve. (I know this is suboptimal and could be read from config file. But hey ! I am a beginner).
* Change the IP (say from 0.0.0.0 to 127.0.0.1) and Port (say 3030:8080) per your needs. (Again can be read from a config file)
* Run `cargo run`

## API
* WebUI can be accessed at http://IP:PORT/web/ls
* Fetch the list of directories at `my/dir` -> `GET http://IP:PORT/api/ls?path=my%2Fdir`
* Download the file at `my/dir/file.ext` -> `GET http://IP:PORT/api/download?path=my%2Fdir%2Ffile.ext`

## Dependencies
* warp, hyper, tokio
* [sailfish](https://github.com/Kogia-sima/sailfish) for web templating.

## Usable Alternatives
* [miniserve](https://github.com/svenstaro/miniserve)
