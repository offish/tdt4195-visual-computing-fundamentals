# Assignment 1-3
Forked from [gloom-rs](https://github.com/pbsds/gloom-rs)


<!-- To get started, make sure you have `git`, `cargo` and, `rustc` installed and available.

	git clone https://github.com/pbsds/gloom-rs
	cd gloom-rs
	cargo run


## GLM

We use a variant of GLM known as [nalgebra-glm](https://docs.rs/nalgebra-glm/0.15.0/nalgebra_glm/), which differs *slightly* from the standard GLM library.


## Report

You're free to write your report any way you'd like, as long as it is delivered as a PDF file.

To spread the gospel, I have included a `pandoc` report skeleton in the `report` folder.
To use pandoc, make sure you have `pandoc` installed along with a supported latex engine.
Make sure it works before using it to write your report.

## Cybele

If you're using the lab computers in Cybele, you will be using a network-mounted home directory which is subject to both low quotas and high latency.
To speed up your work we highly reccomend running the following, to put the build directory in RAM rather than on disk:

```shell
test -d target/ && rm -rf target/
ln -s /dev/shm target
```

## Code delivery

We want the following files and folders to be delivered in a ZIP file:

* `resources`
* `shaders`
* `src`
* `Cargo.lock`
* `Cargo.toml`

**Important:** Do not include the `target` folder!

To automatically make an archive (`source.zip`) ready for uploading to blackboard:

* Make sure any extra assets or resources you might have added are located in the `resources` folder
* Then run either:
	* `./create_code_archive_for_blackboard_LINUX.sh`
	* `create_code_archive_for_blackboard_WINDOWS.bat`.

This zip script will explicitly ignore the `target` folder, and the following two of the files given as a handout for exercise 3 (just to save space):

* `resources/helicopter.obj`
* `resources/lunarsurface.obj` -->
