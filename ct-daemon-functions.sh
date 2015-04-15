#!/usr/bin/env bash

source ct.config

log(){
  if [ "true" == "$CTDEBUG" ]; then
	echo -e $@${NC}
  fi
}

printInformations() {
    ISHTTP=-1;
    read PID COMMAND PORTS <<< $1
    PORTS=${PORTS//\*/localhost}
    PORTS=${PORTS//localhost/127.0.0.1}
    for port in $PORTS
    do
        curl -m 5 -s http://$port > /dev/null
        ISHTTP=$?
        ISHTTPS="false"
        if [ "$ISHTTP" -gt "0" ]
        then
            curl -m 5 -s -k https://$port > /dev/null
            ISHTTP=$?
            ISHTTPS="true"
        fi
        if [ $ISHTTP -eq 0 ]
        then
            cwd=`lsof -p $PID -Ffn | grep fcwd -A 1 | grep ^n | awk '/^n/ { print substr($0, 2) }'`
            echo " { 'pid' : '$PID', 'address' : '$port', 'command' : '$COMMAND', 'cwd' : '$cwd', 'secure': $ISHTTPS},"
        fi;
    done
}

getHTTPListening() {
    LISTENING=`lsof -iTCP -sTCP:LISTEN -P -Fcn | awk '/^p[0-9]+/ {printf "\n"; printf substr($1, 2); printf " "} /^c.*/ { printf substr($1, 2); printf " "} /^n.*/ {printf substr($1,2); printf " ";}'`
    echo "["
    while read -r row; do
        printInformations "$row" &
    done< <(echo "$LISTENING")
    wait
    echo "null]"
}

#build a basic homepage with content from CDN for CSS/JS
getHomePage(){
    echo -e "HTTP/1.1 200 OK\r"
    echo "Content-type: text/html"
    echo

    echo "$CONTENT$SCRIPTS"
}

# respond to / request by sending a basic html file with inline javascript that calls /apps
# respond to /scan request with the formatted list of url (with port) detected location on disk (guesstimate) and pid
handleRequest() {
    case "$REQUEST" in
        "/") getHomePage
        ;;
        "/scan") getHTTPListening
        ;;
        *) echo 'Not Found';
        ;;
    esac
}

startServer() {
    #http://stackoverflow.com/questions/16640054/minimal-web-server-using-netcat
    rm -f /tmp/ct_ports_out
    mkfifo /tmp/ct_ports_out
    trap "rm -f /tmp/ct_ports_out" EXIT
    # detect if we have a netcat-traditional or netcat-openbsd
    if nc -q 2>&1 | grep -q "requires an argument";
        then
           echo ${PORT_LIST} 
            CMD_ARGS="-q 0 -l -p ${PORT_LIST}";
        else 
           echo la 
            CMD_ARGS="-l ${PORT_LIST}";
    fi;
    while true
    do
        cat /tmp/ct_ports_out | nc ${CMD_ARGS} > >( # parse the netcat output, to build the answer redirected to the pipe "out".
        export REQUEST=
        while read line
        do
            line=$(echo "$line" | tr -d '[\r\n]')

            if echo "$line" | grep -qE '^GET /' # if line starts with "GET /"
                then
                REQUEST=$(echo "$line" | cut -d ' ' -f2) # extract the request
            elif [ "x$line" = x ] # empty line / end of request
                then
                log "$REQUEST"
                handleRequest > /tmp/ct_ports_out
            fi
        done
        )
    done
}


