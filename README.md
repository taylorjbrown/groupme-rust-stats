A Tool to download all history of a GroupMe chat room
and runs a few stats on each user's messages in the group:

Output:

Two folders will be created when run:
- raw_output:
	- This holds all the raw JSON from the groupme API, file names are the timestamp range for the file's messages
- results:
	- messages.csv:

			| createdTimestamp | userId | userName@timeOfMessage| messageText | totaldabs | favorited_by | message_id |
   
	- users.csv:

			| userId | usernames (separated by comma)
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

		group_ids = `<Groupme_Chat_Id` <br>
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
		- mac binary:
	- Change Settings.default.toml to Settings.toml and add the following information:

		group_ids = `<Groupme_Chat_Id` <br>
		api_key = `<Groupme_Api_Key>` <br>
		output_folder = `<Relative_Path_Loction_For_Output_Folder>` <br>
		results_folder = `<Relative_Path_Location_For_Results_Folder>`

	- Use command: 
		- `groupme-rust-stats`
	- On complete you should see output like those below message:
		![Alt text](readmePic/ExpectedOutPut.png?raw=true "ExpectedOutput")


