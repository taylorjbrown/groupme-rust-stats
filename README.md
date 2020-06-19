A Tool to download all history of a GroupMe chat room
and runs a few stats on each user's messages in the group:

Prerequists:
1) Create an application api key for GroupMe API
	- Go to https://dev.groupme.com/applications
	- Login with your account
	- Click "Create Application"
	- Set callback to "localhost"
	- Click save
	- Copy the value at "<user name>'s Access Token"
		- This will be used in the settings file as the `<Groupme_Api_Key>`
2) Get the group chat id:
	- Go to https://web.groupme.com/chats
	- Login with you account
	- Open the developer console by following one of these steps:
		https://balsamiq.com/support/faqs/browserconsole/
	- Look at the networking tab
	- Find the URL that matches this pattern:
		- https://api.groupme.com/v3/groups/0000000/messages?acceptFiles=1&limit=10
		- 0000000 -> This is the chat's Id, used as the in settings file
		`<Groupme_Chat_Ids>`


Output:

Two folders will be created when run:
- raw_output:
	- This holds all the raw JSON from the groupme API, file names are the timestamp range for the file's messages
- results:
	- messages.csv:

			| createdTimestamp | userId | userName@timeOfMessage| messageText | totaldabs | favorited_by | message_id |
   
	- users.csv:

			| userId | usernames (separated by comma) |
	- results.txt:
		- For each user in group, this file holds the follow stats:
			- Total Likes
			- Top Liked Messages
			- Ratio of amount of messages to likes
			- Who Liked My Messages:
				- An ordered list of other users in the group and their total like count this user's messages
			- Whos Messages Did I Like
				- An ordered list of other users in the group and current user's total like count for their messages


How To Run:
- Option 1: (use Rust runtime)
	- Install Rust runtime time from here: 
		- https://www.rust-lang.org/tools/install
	- Clone repository
	- Change Settings.default.toml to Settings.toml and add the following information:

		group_ids = `<Groupme_Chat_Ids>` <br>
		api_key = `<Groupme_Api_Key>` <br>
		output_folder = `<Relative_Path_Loction_For_Output_Folder>` <br>
		results_folder = `<Relative_Path_Location_For_Results_Folder>`

	- Use command: 
		- `cargo run`
	- On complete you should see output like those below message:

		![Alt text](readmePic/ExpectedOutPut.png?raw=true "ExpectedOutput")
- Option 2: (use binary file and run from command line/terminal)
	- Download binary and settings file: 
		- linux binary:
		- windows binary:
	- Change Settings.default.toml to Settings.toml and add the following information:

		group_ids = `<Groupme_Chat_Ids>` <br>
		api_key = `<Groupme_Api_Key>` <br>
		output_folder = `<Relative_Path_Loction_For_Output_Folder>` <br>
		results_folder = `<Relative_Path_Location_For_Results_Folder>`

	- Use command: 
		- `groupme-rust-stats`
	- On complete you should see output like those below message:

		![Alt text](readmePic/ExpectedOutPut.png?raw=true "ExpectedOutput")


