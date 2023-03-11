
import random
import os
import json
import time

node_format = \
"""    {
        "connections": {connections},
        "name":        {name},
        "uid":         {uid}
    }"""

node_format2 = \
        """    "{uid}": {
        "connections": {connections},
        "name":        {name},
    },"""

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
        return {"connections": list(self.connections), "name": self.name, "uid": self.uid}


    def save( ):
        format1_array = [ node.get_json( ) for node in Node.nodes.values( ) ]
        format2_dict  = Node.save_format2( )

        finished_array = [format1_array, format2_dict]

        return json.dumps( finished_array, ensure_ascii=False, indent=4 )

    def save_format2( ):
        format2_dict = dict()

        for node in Node.nodes.values( ):
            format2_dict[str(node.uid)] = {"name": node.name, "connections": list(node.connections)}

        return format2_dict

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
    for node in json.load( f )[0]:
        Node( node["connections"], node["name"], node["uid"] )

run = True
print( "Commands:"     )
print( "A: add"        )
print( "P: print"      )
print( "Q: quit"       )
print( "S: save"       )
print( "L: list"       )
print( "C: connection" )

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

        print( )
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
                search = input( "enter search > " )
                matches = Node.search( search )
                if len( matches ) == 1:
                    print( "Added", matches[ 0 ].name )
                    connections.append( matches[ 0 ].uid )
                    time.sleep( 1 )


            except KeyboardInterrupt:
                adding = False
                print( )

        Node( connections, name )
        print( "Added Node" )
        continue

    if command == "L":
        for uid in Node.nodes.keys( ):
            print( f"{uid} : {Node.nodes[uid]}" )

        continue

    if command == "C":
        print( "--- ADD CONNECTION ---" )

        searching = True
        while searching:
            search = input( "person one > " )
            matches = Node.search( search )
            if len( matches ) == 1:
                print( "Person 1 Selected", matches[ 0 ].name )
                person1 = matches[ 0 ]
                searching = False
                break

            print( f"{len(matches)} matches found" )

        searching = True
        while searching:
            search = input( "person two > " )
            matches = Node.search( search )
            if len( matches ) == 1:
                print( "Person 2 Selected", matches[ 0 ].name )
                person2 = matches[ 0 ]
                searching = False
                break

            print( f"{len(matches)} matches found" )

        person1.connections.add( person2.uid )
        person2.connections.add( person1.uid )

print( Node.save_format2() )





