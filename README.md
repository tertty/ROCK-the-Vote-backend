
# ROCK-the-Vote-backend

Web Server element of the ROCK the Vote Pebble Watchapp.

ROCK! The Vote is a Pebble application that allows you to voice your opinion, tip the scales, and influence those around you. Each day (12am EST), users will be able to log into the app and see the new prompt to vote on. Each user can vote once, siding with one of the two options they most agree with. After a valid vote has been cast, users can compulsively check throughout the day, seeing which option is in the lead.

## Building Docker Image and Self-hosting
The Dockerfile is set up to automatically start the web server and serve up the client when building the image.

To build the image, run the following command in the project's directory:
`docker build -t rtv-web-image .`

then:
`docker compose up`

To view the console logs from within the container:
`docker logs -f <container_name_or_id>`

### Docker Environment Variable

| Variable Name | Description |
|--|--|
| ROCKET_ADDRESS | Domain website/websocket will be hosted on, i.e. localhost, website.com |
| ROCKET_PORT | Port website/websocket will run on, i.e. 8080 |

## Gotchas
- Currently, the SQLite database persists a container down and up. This is great for preserving the tally of what users picked over time. This isn't great because there's currently a bug with the way the questions are injected so you need to start a fresh database each container up and down. This will be addressed... soon™. 
	- In the meantime, if a container goes down make sure you run `docker compose down` before you bring it back up.
- Questions can currently only be edited by adding a raw entry in the 12 vectors found in the PersistentData->new() function. This is not very idiomatic and makes things very predictable if you upload the program's source. 
	- For these reasons, support for an external, loadable question configuration file will be added... soon™.
