
import random
import os
import json

node_format = \
"""    {
        "connections": {connections},
        "name":        {name},
        "uid":         {uid}
    }"""

class Node:

    nodes = { }

    def __init__ ( self, connections, name, uid=None ):
        self.connections = set( connections )
        self.name = name
        if uid == None:
            self.uid = random.randint( 0, 2**32-1 ) # Generate a unique id for the node

        else:
            self.uid = uid

        for node in self.get_connections( ):
            node.connections.add( self.uid )

        Node.nodes[ self.uid ] = self

    def get_connections( self ):
        connections = []
        for connection in self.connections:
            try:
                connections.append( Node.nodes[ connection ] )

            except KeyError:
                pass

        return connections

    def get_json( self ):
        return node_format \
        .replace( "{connections}", str( list( self.connections ) ) ) \
        .replace( "{name}", '"' + self.name + '"' ) \
        .replace( "{uid}", str( self.uid ) )
        # this is bad, like really really bad but it just converts to json

    def save( ):

        json_array = [ node.get_json( ) for node in Node.nodes.values( ) ]

        json = "[\n" + ",\n".join( json_array ) + "\n]"

        return json

    def __str__( self ):
        return self.name

    def __repr__( self ):
        return __str__( self )

    def search( phrase ):
        matches = [ ]
        for key in Node.nodes.keys( ):
            name = Node.nodes[key].name
            if phrase.lower( ) in name.lower( ):
                matches.append( Node.nodes[key] )

        return matches

with open( "people.json", "r" ) as f:
    for node in json.load( f ):
        Node( node["connections"], node["name"], node["uid"] )

run = True
print( "Commands:" )
print( "A: add"    )
print( "P: print"  )
print( "Q: quit"   )
print( "S: save"   )
print( "L: list"   )

while run:

    command = input( "\nenter command > " )

    if command == "Q":
        run = False
        continue

    if command == "P":

        print( "--- SEARCH ---" )

        havent_found_a_match = True
        while havent_found_a_match:
            search = input( "enter search > " )
            matches = Node.search( search )
            if len( matches ) == 1:
                match: Node = matches[ 0 ]
                havent_found_a_match = False
                break

            if len( matches ) == 0:
                print( "no matches" )
                continue

            print( ", ".join( [match.name for match in matches] ) )

        print( "--- INFORMATION ---" )
        print( "name:", match.name )
        print( "connections:", ", ".join( [ connection.name for connection in match.get_connections( ) ] ) )
        print( "uid:", match.uid )
        continue

    if command == "S":
        with open( "people.json", "w" ) as f:
            f.write( Node.save( ) )

        print( "Saved" )
        continue

    if command == "A":
        print( "--- ADD NODE ---" )
        name = input( "enter name > " )

        connections = []
        adding = True
        while adding:
            try:
                os.system( "clear" )
                search = input( "enter search > " )
                matches = Node.search( search )
                if len( matches ) == 1:
                    print( "Added", matches[ 0 ].name )
                    connections.append( matches[ 0 ].uid )
                    input( )

            except KeyboardInterrupt:
                adding = False


        Node( connections, name )
        print( "Added Node" )
        continue

    if command == "L":
        for uid in Node.nodes.keys( ):
            print( f"{uid} : {Node.nodes[uid]}" )

        continue




