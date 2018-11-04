# Spect

A plugin-based web monitor for arbitrary jobs

Spect is basically a webserver that supports plugins for utility pages that print internal system state. The main application installs "SpectSubpageModules" on initialization that know how to introspect into running state, and can expose that state on some path.

The canonical subpage example is the ZcfgSubpage, which enumerates the supported zcfg flags (and soon their set values).
