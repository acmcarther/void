Wayland Wall
=========================

Wayland Wall is a collection of protocols, called bricks, to create a complete desktop experience for Wayland.

The idea is copied from [wayland-protocols](https://cgit.freedesktop.org/wayland/wayland-protocols/) but for protocols less likely to make it to the major DEs.


Releases
--------

You can find releases tarballs [here](https://www.eventd.org/download/wayland-wall/).


Protocols
---------

We currently have this list of protocols:

*   background
    This protocol will allow clients to display a surface as your screen background
*   dock-manager
    This protocol will allow clients to dock surfaces on screen edges
*   launcher-menu
    This protocol will allow launcher and menu clients, with a proper implicit grab
*   notification-area
    This protocol will allow notification daemons to display their notifications using Wayland.
*   window-switcher
    This protocol will allow privileged clients to list, switch to and close other clients surfaces


Compositor implementations
--------------------------

For now, these compositors are supported, with these protocols:

* Weston, in [weston-wall](https://github.com/wayland-wall/weston-wall)
    * background
    * dock-manager
    * launcher-menu
    * notification-area
    * window-switcher (bare implementation, no workspaces support)
* Orbment, in [orbment-wall](https://github.com/wayland-wall/orbment-wall)
    * background
    * notification-area


Client implementations
----------------------

* background:
    * ww-background, a simple demo (build Wayland Wall with `--enable-clients` and optionally `--enable-images`)
* dock:
    * ww-dock, a simple demo (build Wayland Wall with `--enable-clients` and `--enable-text`)
* launcher-menu:
    * [rofi](https://github.com/DaveDavenport/rofi/tree/wip/wayland) (experimental)
* notification-area:
    * [eventd](https://www.eventd.org/)
* window-switcher:
    * [rofi](https://github.com/DaveDavenport/rofi/tree/wip/wayland) (experimental)
