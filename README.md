# hosts file to block popcorn palace/vrclinking posters & telemetry

tired of toggling off the posters every time? don't want your location to be tracked?

**this does not modify your client**

ads that are baked into the world aren't removed, and for worlds like popcorn palace, other domains like vr-m.net probably still have telemetry built-in.

## usage

### automatic

1. download the proper executable for your OS from the [releases page](https://github.com/artifishvr/vrc_adblock/releases)
    1. please make a backup of your hosts file (`C:\Windows\System32\drivers\etc\hosts` or `/etc/hosts`) if you care about it, just in case.
2. run the executable as admin/root (it will prompt you if needed)
3. your hosts file is updated, enjoy

if the txt on github is updated, you can just run the script again to update the hosts file, you shouldn't need a new release.

### manual

#### pi-hole

1. Go to Pi-hole admin → **Settings** → **Adlists**
2. Add the raw URL `https://raw.githubusercontent.com/artifishvr/vrc_adblock/refs/heads/main/blocked_hosts.txt`
3. Run `pihole -g` to update

#### windows

1. open notepad as admin
2. open `C:\Windows\System32\drivers\etc\hosts` and add the contents of [blocked_hosts.txt][hosts-link] at the bottom (on a new line)
3. done! enjoy.

#### linux (by @butterroach)

1. execute your text editor as the root user
2. carefully instruct your text editor to begin the immediate modification of the path `/etc/hosts`
3. instruct the cursor currently visible on the afformentioned editor to navigate to the very end of the path's plain text contents
4. on your browser, navigate to the extremely important [blocked_hosts.txt][hosts-link] file
5. instruct the browser cursor (not to be confused with the cursor of the textual editor) to select the contents of the file
6. i'm getting tired of writing this just copy and paste it to the end of the file
7. ok

[hosts-link]: https://github.com/artifishvr/vrc_adblock/blob/main/blocked_hosts.txt
