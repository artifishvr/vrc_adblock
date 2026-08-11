# ad blocker for popcorn palace posters

tired of toggling off the posters every time?

blocks the domain that the posters are loaded from, which causes them to not load even their models.

ads that are baked into the world aren't removed.

## usage

### script

1. download the proper executable for your OS from the [releases page](https://github.com/artifishvr/vrc_adblock/releases)
    1. please make a backup of your hosts file (`C:\Windows\System32\drivers\etc\hosts` or `/etc/hosts`) just in case.
2. run the script as admin/root (it will prompt you if needed)
3. your hosts file is updated, enjoy

if the txt on github is updated, you can just run the script again to update the hosts file

### manual

#### windows

1. open notepad as admin
2. open `C:\Windows\System32\drivers\etc\hosts` and add the contents of [blocked_hosts.txt][hosts-link] at the bottom (on a new line)
3. done! enjoy.

#### linux

1. execute your text editor as the root user
2. carefully instruct your text editor to begin the immediate modification of the path `/etc/hosts`
3. instruct the cursor currently visible on the afformentioned editor to navigate to the very end of the path's plain text contents
4. on your browser, navigate to the extremely important [blocked_hosts.txt][hosts-link] file
5. instruct the browser cursor (not to be confused with the cursor of the textual editor) to select the contents of the file
6. i'm getting tired of writing this just copy and paste it to the end of the file
7. ok

[hosts-link]: https://github.com/artifishvr/vrc_adblock/blob/main/blocked_hosts.txt
