<p align="center">
  <img width="500" height="500" src="./images/pinepods-logo.jpeg">
</p>

# PinePods

- [PinePods](#pinepods)
  - [Features](#features)
  - [Hosting](#hosting)
  - [Installing/Running](#installingrunning)
  - [ToDo](#todo)
    - [Needed pre-beta release](#needed-pre-beta-release)
    - [To be added after beta version](#to-be-added-after-beta-version)
  - [Platform Availability](#platform-availability)
  - [API Notes](#api-notes)
  - [Screenshots](#screenshots)
      
PinePods is a Python based app that can sync podcasts for individual accounts that relies on a central database with a web frontend and apps available on multiple platforms

## Features
Pinepods is a complete podcasts management system and allows you to play, download, and keep track of podcasts you enjoy. It allows for searching new podcasts using The Podcast Index and provides a modern looking UI to browse through shows and episodes. In addition, Pinepods provides simple user managment and can be used by multiple users at once using a browser or app version. Everything is saved into a Mysql database including user settings, podcasts and episodes. It's fully self-hosted, and I provide an option to use a hosted API or you can also get one from the podcast API and use your own. There's even many different themes to choose from! Everything is fully dockerized and I provide a simple guide found below explaining how to install Pinepods on your own system. 

## Hosting
N/A

## Installing/Running
N/A

## ToDo

Added in order they will be completed:

 - [x] Create Code that can pull Podcasts
 - [x] Integrate Podcast Index
 - [x] Play Audio Files using Python - The python vlc package is used for this
 - [x] Record listen history and display user history on specific page
 - [x] Record accurate listen time. So if you stop listening part-way through you can resume from the same spot
 - [x] Scrubbing playback from a progress bar - ft.slider()
 - [x] Add visual progress bar based on time listened to podcasts partly listened to
 - [x] Add Download option for podcasts. In addition, display downloaded podcasts in downloads area. Allow for deletion of these after downloaded
 - [x] Add Queue, and allow podcasts to be removed from queue once added (Queue is added but you can't remove them from it yet)
 - [x] Create login screen
 - [x] Check for and remove podcasts no longer available (This will be handled from scheduled cron job that queues)
 - [x] Check user values when adding new user
 - [x] Prevent user from being added without required info 
 - [x] Prevent submit for user from being hit without populated values
 - [x] Figure out why some podcasts don't appear in search (This was because of the old podcast index python package. Rebuilt using requests and now it works great)
 - [x] Implement resume playback throughout all areas of the app
 - [x] Implement Episode view (Should be able to display html via markdown)
 - [x] Theme settings
 - [x] Fix issues with episodes playing not in database (Sorta fixed. For now episodes played are always in database. External to database episodes coming soon)
 - [x] Add picture of current episode to soundbar
 - [x] Fix issue with podcasts sometimes not registering time when played (Occured becuase of VLC not registering time. It now tries 5 times and always works)
 - [x] Implement smoother scrolling with big list loading (Mostly fixed. If there's a podcast with hundreds of episodes with loads of markdown rendered it can still cause slowdown. Moving this to the backlog.)
 - [x] Admin area for User management
 - [x] Add new user currently doesn't set admin or not. Just NULL (It now sets non admin by default)
 - [x] Make Admin options not available to standard users
 - [x] Ability to Delete Users
 - [x] Ensure there is always at least one admin user
 - [x] Allow guest user to be disabled
 - [x] Ensure changes cannot be made to guest user
 - [x] Ensure Users cannot delete themselves
 - [x] Guest sign in via button on login screen when enabled
 - [x] Episode Streaming via external web client doesn't currently work (Fixed, mostly. I now use flet audio controls to do everything)
 - [x] Implement saved episodes view
 - [x] On hover user hello
 - [x] Add caching to image server
 - [x] User self service creation
 - [x] User container click stats page
 - [x] Implement download episode checking throughout
 - [x] Implement saved episode checking throughout
 - [x] Add loading wheels throughout
 - [x] Add verification snack bars throughout 
 - [x] Finish Themes
 - [x] Remove Podcasts from search or just don't allow adding a second time (It throws a snackbar if you try and add one a second time)
 - [x] Removing a podcast currently doesn't display snackbar
 - [x] Implement sign in retention. Cookies sort of (App retention now workss. It creates session keys and stores them locally. Browser retention is next.)
 - [x] Audio volume interaction (implemented but layout is still wrong)
 - [x] Layout soundbar better (it adjusts for screensize but can overlap at times with the episode title)
 - [x] Create Web App
     - [x] More responsive layout 
     - [x] Security and Logins
     - [x] Database interaction for users and podcast data

 ### Needed pre-beta release
 - [ ] Fully update Readme with updated info and docs including deployment guide
 - [ ] Bugs
    - [x] Links when searching an episode are blue (wrong color)
    - [x] When changing theme, then selecting 'podcasts' page, the navbar does not retain theme
    - [x] There's an issue with Queue not working properly. Sometimes it just plays instead of queues (Fixed when switching to flet audio control)
    - [x] Clicking podcast that's already been added displays add podcast view with no current way to play
    - [x] Clicking play buttons on a podcast while another is loading currently breaks things
    - [x] Pausing audio changes font color
    - [x] Login screen colors are wrong on first boot
    - [x] Themeing currently wrong on audio interaction control
    - [x] Starting a podcast results in audio bar being in phone mode on application version (This should be fixed. I load the check screensize method now further down the page. Which results in consistent width collection.)
    - [x] Starting a podcast results in audio bar being in phone mode on application version
    - [x] Adding a podcast with an emoji in the description currently appears to break it
    - [x] Layout breaks when pausing for podcast names
    - [x] The queue works but currently does not remove podcasts after switching to a new one
    - [x] Resume is currently broken (it now works but it double plays an episode before resuming for some reason. It still double plays and there's not a great way to fix it. Return later. Updates to flet are likely to help eventually)
    - [x] Double check 2 users adding the same podcast (There was an issue with checking playback status that is now fixed)
    - [x] After refresh auto update current route
    - [ ] Double and triple check all interactions to verify functionality
    - [x] Fix any additional browser playback bugs (Audio now routes properly through the proxy)
 - [ ] Dockerize
     - [x] Package into Container/Dockerfile
     - [x] Pypods image in docker hub
     - [x] Create Docker-Compose Code
     - [ ] Mixed content - Currently running http or https content can cause an error
     - [ ] Option to run your own local podcast index api connection


 ### To be added after beta version

 - [ ] Implement Gravitar API for profile picture
 - [ ] Rotating currently playing
 - [ ] Suggestions page - Create podcasts you might like based on the ones you already added
 - [ ] Allow local downloads to just download the mp3 files direct
 - [ ] Page refreshing to handle adding and removing of things better
 - [ ] Handle Images better. Currently it takes a long time to parse through many images (Needs to not load all images. Only ones on screen)
 - [ ] Reload not needed to add and remove episodes from pages
 - [ ] Customizable login screen
 - [ ] Add highlight to indicate which page you're on
 - [ ] Add Itunes podcast API
 - [ ] Better queue interaction. There should be a way to drop down current queue and view without changing route
 - [ ] MFA Logins - Github integration and local MFA (OAuth)
 - [ ] Implement Browser edition sign in retention (This will require some kind of OAuth provider. Part of OAuth and MFA)
 - [ ] GUI Wrapper for App
     - [ ] Server Hosting and client Interaction - Client interaction works via API with mariadb which is hosted on server side
     - [ ] Linux App
     - [x] Proper web layout
     - [ ] Windows App
     - [ ] Mac App
     - [ ] Mobile Apps
       - [ ] Sign in retention for moble editions
       - [ ] Android App
       - [ ] IOS App
  - [ ] Add verification before deleting user
  - [ ] Rating System
  - [ ] Sharing System


## Platform Availability

The Intention is for this app to become available on Windows, Linux, Mac, Android, and IOS. The server will be run from docker and connect to the clients on these platforms

## API Notes

Coming soon

## Screenshots

<p align="center">
  <img src="./images/podlist.png">
</p>