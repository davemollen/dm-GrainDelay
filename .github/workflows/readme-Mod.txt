To install the MOD plugin, copy the .lv2 folder to your MOD:

- Open the terminal and run "scp -O -rp <path-to-plugin.lv2> root@moddwarf.local:/root/.lv2/" to copy the lv2 folder to your MOD. 
Don't forget to replace <path-to-plugin.lv2> with the correct path.
- If this is the first time you connect to your MOD via ssh you will get the following prompt: 
“Are you sure you want to continue connecting (yes/no/[fingerprint])?”. 
Answer this with “yes”.
- When asked for the root@moddwarf.local’s password, enter “mod”.
- The plugin files should have been successfully copied to your MOD. You might need to reboot your MOD to get the plugin to show up.

