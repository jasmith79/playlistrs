# playlistrs
Extracts playlist data from iTunes®/Music Libraries. Tested against iTunes

## Note About Catalina and later:

Starting in MacOS Catalina, iTunes® has been replaced with the new Music app. While the new Music app includes the ability to export to m3u or m3u8, but if like me you have 50+ playlists it's a little inconvenient. You can automate the exporting with Automator but I prefer to just use this tool which can also change the path for other devices.

## Usage

FIrst you'll need to dump your iTunes/Apple Music library because as far as I can tell Apple does not expose an API for
tool to grab it directly from your local music data. You'll want to open iTunes/Music, then use the context menu to go 

> File -> Library -> Export Library

![Context menu image, shows using the File option to export Library.xml](https://github.com/jasmith79/playlistrs/blob/main/resources/Image%202-11-24%20at%202.58%20PM.jpeg?raw=true)

It doesn't really matter where you save it, but you'll need to remember and it should be someplace that you can easily
find it from the command line in Terminal. Here I'm saving it to the desktop:

![File dialog image, shows saving Library.xml to the MacOS Desktop](https://github.com/jasmith79/playlistrs/blob/main/resources/Image%202-11-24%20at%202.59%20PM.jpeg?raw=true)

You can then fire up Terminal or whatever terminal emulator you use and pass the path to the `Library.xml` that you
dumped a second ago:

![Terminal emulator image, shows invoking playlistrs with the path to the Library.xml](https://github.com/jasmith79/playlistrs/blob/main/resources/Image%202-11-24%20at%203.19%20PM.jpeg?raw=true)

For all of the available options see `--help` or browse the source code.
