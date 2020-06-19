A Tool to download all history of a GroupMe chat room
and runs a few stats on each user's messages in the group:

Prerequisites:
1) Create an application api key for GroupMe API
	- Go to https://dev.groupme.com/applications
	- Login with your account
	- Click "Create Application"
	- Set callback to "localhost"
	- Click save
	- Copy the value at `<user name>'s Access Token`
		- This will be used in the settings file as the `<Groupme_Api_Key>`
2) Get the group chat id:
	- Go to https://web.groupme.com/chats
	- Login with you account
	- Open the developer console by following one of these steps:
		https://balsamiq.com/support/faqs/browserconsole/
	- Look at the networking tab
	- Find the URL that matches this pattern:
		- https://api.groupme.com/v3/groups/0000000/messages?acceptFiles=1&limit=10
		- 0000000 -> This is the chat's Id, used in settings file as
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
				- An ordered list of the other users in the group with a total like count on this user's messages
			- Whos Messages Did I Like
				- An ordered list of other users in the group with the current user's total like count for their messages


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
	- On complete you should see an output like the one below:

		![Alt text](readmePic/ExpectedOutPut.png?raw=true "ExpectedOutput")
- Option 2: (use binary file and run from command line/terminal)
	- Download binary from the releases (supports windows and linux [compiled on ubuntu 18]): 
		- https://github.com/taylorjbrown/groupme-rust-stats/releases/latest
	- Create a folder, add the downloaded binary to it
	- Create a Settings.toml in the same folder as the binary
	- Add the following information to the settings file:

		group_ids = `<Groupme_Chat_Ids>` <br>
		api_key = `<Groupme_Api_Key>` <br>
		output_folder = `<Relative_Path_Loction_For_Output_Folder>` <br>
		results_folder = `<Relative_Path_Location_For_Results_Folder>`

	- Open command-line/terminal, navigate to folder that holds the binary
	- Run the following command: 
		- `./groupme-rust-stats`
	- On complete you should see an output like the one below:

		![Alt text](readmePic/ExpectedOutPut.png?raw=true "ExpectedOutput")
- Option 3: (install from cargo package manager)
	- Have rust runtime installed on machine
	- Run `cargo install groupme-rust-stats`
	- Add a Settings.toml file, in current folder, with the following details:
	
	
		group_ids = `<Groupme_Chat_Ids>` <br>
		api_key = `<Groupme_Api_Key>` <br>
		output_folder = `<Relative_Path_Loction_For_Output_Folder>` <br>
		results_folder = `<Relative_Path_Location_For_Results_Folder>`

	- Run package with `~/.cargo/bin/groupme-rust-stats`


