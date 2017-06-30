# Blograph

[![Travis](https://img.shields.io/travis/passcod/blograph.svg)](https://travis-ci.org/passcod/blograph)
[![Coveralls](https://img.shields.io/coveralls/passcod/blograph.svg?style=flat-square)](https://coveralls.io/github/passcod/blograph)

_Blogging done my way._

## Overview

Blograph's model is that posts have metadata and content. Metadata can include
a list of parent posts. Posts are organised in Lists, and there's always a
master List of all the posts in the source. That is then filtered down to lists
for e.g. the frontpage, the tags, the parents and children of a post, etc.
Posts (and Lists) are immutable. The only variable is the current time, because
posts can be dated to the future.

Once the Source is first read by the library, that's it, there's no more
dependency on files on disk. Systems engineering tells you why this is
important: RAM access is **four times faster** than SSD access. But also, it
means the files can change on disk without the program being affected in the
least. Thus, I can update the Source files on disk while the server is running,
and only _then_ reload, and the server will never be in an inconsistent state.

The name is rather simple: through parents and children posts, a Blograph is
not only a chronological collection of posts, but contains directed **graphs**.

### Source and parsing

At the source, **posts** are markdown files in a repository (or a folder). The
relative path and filename of a post are parsed for an **optional** date in
either YYYY-MM-DD or YYYY-MMM-DD format (e.g. `2017-02-25` or `2017-feb-25`),
then the rest up to the first dot is considered the **slug** of the post.

Inside the file, posts may have **front matter**, a YAML metadata section
delimited by `---` above and below it. If a file has front matter, the first
`---` must be the first line.

The front matter can contain arbitrary metadata, but some things are handled
in a defined manner:

- The `date` field is a string containing an ISO8601 UTC date (month/date
  `2017-01-23` or week/day `2017W042` or ordinal `2017023`), or an ISO8601 UTC
  datetime (also supporting the previous date formats) finishing with `Z`, or
  an ISO8601 datetime with timezone indicator in the numeric format (`+13:00`
  or `-01:00`). It **takes precedence** over the date parsed from the
  path/filename.

- The `tags` field is an array of strings. The **first** tag is special, as it
  specifies the **kind** of the post. E.g. it can be a "poem" also tagged with
  "roses" and "violets".

- The `parent` field is a string containing the **slug** of a single post that
  is the parent of this one.

- The `parents` field is an array of strings, each of the **slug** of a post
  that is one of the parents of this one. The `parents` field takes precedence
  over the `parent` field if both are specified.

- The `title` field is a string that overrides the **title** of the post.
  Without this field, the post's title is derived from its **slug**.

- The `page` field is a boolean that specifies whether a post is a "page" or
  not. Pages are not shown on the frontpage and semantically are meant to be
  standalone and undated. However, they *can* have a date. A post is
  automatically a page if it has no date.

- The `author` field is a string.

### A rusty backend

The Rust part of Blograph handles loading from disk, parsing path/filenames and
metadata, and rendering the markdown content. It is split into three crates:
one handling posts and metadata; another handling lists, relationships, and
iteration; the third (the top level crate) handling reading an entire folder
and subfolders for posts, as well as the **Neon** binding.

The Rust List type contains `Arc<Post>`s, which provides a strong guarantee
that there is only a single instance of a Post in memory, no matter how many
Lists it belongs to. Lists simply hold a reference to the Post, and it is
atomically counted and available across threads. Because in Blograph posts are
immutable, there is no need to ever obtain a write lock, which simplifies
synchronisation code.

### A flashy interface

The Rust post, metadata, list, and directory loader are all exposed through
Neon as a Node.js API. The first three are classes, while the last is a single
function returning a List.

For performance and implementation reasons, not all the functionality of the
Rust models are available through the Neon interface. Also, the Neon classes
are limited to simple methods, and do not have getters or setters. Neon classes
also may not be constructed around types with custom lifetimes, which means
they can't make use of native Rust iterators in a lazy way (yet?).

### A lovely frontend

Thus, an additional layer of JS is implemented on top, wrapping the Neon
classes in JS classes that are written and interact with each other in a much
more idiomatic way. That layer also re-implements functionality missing from
the Neon-powered interface, and adds some additional features.

Finally, an Express.js application serves lists and posts in a nice HTMLy form.

### A stately (re)loader

Blograph actually starts with an empty posts list. Upon starting, it clones a
repository and loads all the posts from that. But it also has an endpoint that
triggers a reload of this repository. A webhook on the repository is attached
to that, and tada! The blog is automatically updated every time something is
pushed to the repo, even when force-pushed!

The Rust backend grants us _seriously fast_ loads. After the git clone is done,
it takes about a second to load a hundred posts… and then that's it, no more
disk I/O required. It feels **instant**.

## Can I use this?

Probably not. This is very heavily opinionated for me. If you do want to use
any part or all of it, feel free to fork it! It's licensed under ISC, so just
go at it.

## Why Rust stable?

The history of this project is that in late 2013, I created the first version
of Blograph in Ruby. Then I set it running and tweaked it until it got to a
"final" version in late 2014. And then I didn't touch it for _two years_. It
ran for two years uninterrupted. When I picked it up again in late 2016 to fix
a few bugs that finally annoyed me enough, it just refused to work on my local
machine. The Ruby engine, all the dependencies, _something_ changed in those
two years and figuring out the right configuration was taking way too long.

Rust **stable** makes a strong guarantee that code written today will still
compile and work with a compiler released years from now, as long as it's still
the same major version. The Rust team is really strict on semver adherence in
that way. Using nightly doesn't have that guarantee at all.

This means that if I leave this project alone and don't touch it for a few
years, I have strong belief that when I eventually do pick it up, as long as I
use the latest Rust **1.x**, it will compile, and it will work. It also means
that whatever performance improvements the compiler picks up in the meantime, I
can get for free, without even having to update my code to work with the latest
Rust version.

This blog has happily lasted four years on one engine so far. I am confident
that it will last at least another four on this new one. I would like it if I
didn't have to rewrite it all when I do pick it up again.
