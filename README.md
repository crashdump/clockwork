# clockwork

Clockwork is a timer-based action release mechanism. In short, it'll do something after some time if you haven't re-armed the mechanism.
The "something" is modular and can be anything you'd like.

## run

First, configure your tasks in `clockwork.toml`. Note: the username and password is used for authenticating with the REST API. 

    [auth]
    username = "foo"
    password = "bar"
    
    [task]
    name = "test"
    timeout = 10
    command.name = "echo"
    command.args = "hello, world!"

Then start the server

    $ clockwork ./clockwork.toml

    run_expired_tasks(): wait until the next task run.
    🚀 Rocket has launched from http://127.0.0.1:8000


Once the server runs, you can query the status of the process through its rest api, for example:

    $ curl -H "Authorization: Basic [REDACTED]" http://127.0.0.1:8000/api/v1/tasks/test

    { 
      "name":"test",
      "status":"counting_down",
      "expires_in":8,
      "command":"echo",
      "result":"pending execution"
    }


another example, with a task having reached 0.

    $ curl -H "Authorization: Basic [REDACTED]" http://127.0.0.1:8000/api/v1/tasks/test

    {
      "name":"test",
      "status":"action_successful",
      "expires_in":0,
      "command":"echo",
      "result":"sucesfully ran echo"
    }