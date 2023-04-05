[![Rust test](https://github.com/Gioelio/Expire-Folder/actions/workflows/test.yml/badge.svg)](https://github.com/Gioelio/Expire-Folder/actions/workflows/test.yml)
[![Latest Stable Version](https://img.shields.io/github/v/release/Gioelio/Expire-Folder?display_name=tag&include_prereleases&sort=semver)]()
[![Download](https://img.shields.io/github/downloads/Gioelio/Expire-Folder/latest/total)]()
[![License](https://img.shields.io/github/license/Gioelio/Expire-Folder)]()

# Expire Folder - WIP

---

## About

The ``exp`` command line aims to handle abandoned projects' dependencies for developers.
This project can also be used for other purposes.

## Status

Currently, the project is a work in progress, although there are some features that are already implemented in the very first release:
- The ``add`` command with the path of a folder or file and an expiration time (specified with the tags ``-d`` for days, ``-m`` for months, ``-y`` for years)).
- The ``list`` command to list the elements tracked by the tool (with the flag ``-remove`` the expired element can be removed from the list and with the flag ``-all`` all the elements are listed).

The aim of the project is to get the following features implemented:
- The ``add`` command with the path of a folder or file and an expiration time (optionally also with a flag for automatic deletion or asking for permission).
- The ``list`` command to list the folders tracked by the tool. 
- The ``remove`` command to remove elements from the _expiration list_.

Future features that I want to implement include:
- A command for the automatic scan of folders with names such as "_target_", "_build_", or "_node_modules_"
- A daemon that remove the expired folders automatically
- An option to put the deleted elements in the Trash folder

## Contributing

To contribute to the project, fork the project and open a pull request. If you don't know what needs to be done, please 
[send me an email](mailto:gioele.fiorenza2000@gmail.com?subject=Feature%20request%20for%20the%20Expire%20folder%20project&body=Hi,%20I'm%20interested%20in%20helping%20you%20with%20the%20Expire%20Folder%20project.%20What%20can%20I%20do?)
 and I'll open an issue on GitHub.