CT : CLI Helper tool
==

This tool is a simple wrapper allowing to have aliases per project, aliases that can be called from whenever the project structure.

Setup
--
You will need a few command line tools for this project to work :

 * bash
 * sed (gnu-sed for Mac users `brew install gnu-sed --default-names`
 * grep
 * dos2unix for Cygwin users
 
Clone the current repository or raw-download the `ct` shell script. Then put it in your path (typical location is ~/bin or /usr/local/bin).

Configure a project
--
Create a `.ctproject` file at the root of your project structure, this file will contain the aliases you want to set for your current project.
It have to look like the following :

    # comments are ignored
    run='mvn clean install jetty:run'
    dbg='mvnDebug clean install jetty:run'
    test='mvn test'
    
Use it
--
Simply call `ct` followed by your alias to launch the command in your current directory. If you execute the command without specifying a command, help screen with the available commands will be printed.

Protip
--
Use consistent aliases in your `.ctproject` files, this way, you can define global aliases for your shell that will allow you to use consistent shortcut regardless of the project type you're on.

For example, you can add an alias `alias run="ct run"` and in each project define such a command. In your shell, a `run` will launch your project, no matter if the underlying task is a Maven or a Grunt one.

License
--
Copyright 2014 Code-Troopers.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this work except in compliance with the License. You may obtain a copy of the License in the LICENSE file, or at:

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
