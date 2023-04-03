# Expire Folder - WIP

---

## About

The ``exp`` command line aims to handle abandoned projects' dependencies for developers.
This project can also be used for other purposes.

## Status

Currently, the project is a work in progress, so here is a subset of the features that I hope to introduce before tagging the first release:
- The ``add`` command with the path of a folder or file and an expiration time (optionally also with a flag for automatic deletion or asking for permission)
- The ``list`` command to list the folders tracked by the tool (and if the aforementioned tag is present, this command creates a list of expired folders that need confirmation to delete)
- The ``remove`` command to unlist elements from the _expiration list_.

Future features that I want to implement include:
- A command for the automatic scan of folders with names such as "_target_", "_build_", or "_node_modules_"

## Contributing

To contribute to the project, fork the project and open a pull request. If you don't know what needs to be done, please 
[send me an email](mailto:gioele.fiorenza2000@gmail.com?subject=Feature%20request%20for%20the%20Expire%20folder%20project&body=Hi,%20I'm%20interested%20in%20helping%20you%20with%20the%20Expire%20Folder%20project.%20What%20can%20I%20do?)
 and I'll open an issue on GitHub.