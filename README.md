# rbuild 

A lightweight, Rust-powered build automation system inspired by tools like PKGBUILD, Make, and modern CI/CD pipelines.

## Roadmap

- Remove temporary files

## Done Things
- Create temporary build workspace
- Detect project type
- Build project
- Install generated binaries

## Long Term Vision

The final goal:

```bash
rb install github.com/user/project
```

should 

```
Clone
 |
Detect
 |
Build
 |
Install
 |
Register
 |
Cleanup
```