# Session layout (Phase A)

Unix user: `rupture`  
Home: `/home/rupture`  
Compositor: labwc  
Daemon: `ruptured` started from `~/.config/labwc/autostart`

```
~/.config/labwc/
  rc.xml            minimal, no human keybind religion
  autostart         exec ruptured

/var/lib/rupture/
  sessions/         JSONL
  memory/

/run/rupture/
  rupture.sock
```

Autologin via greetd `initial_session` or equivalent. No greeter for a human in the default image.

The owner reaches the machine through the prompt channel, not through GDM.
