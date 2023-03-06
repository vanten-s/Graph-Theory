
import random

node_format = \
"""    {
        connections: {connections},
        name:        {name},
        uid:         {uid},
    },
"""

class Node:

    nodes = { }

    def __init__ ( self, connections, name ):
        self.connections = set( connections )
        self.name = name
        self.uid = random.randint( 0, 2**32-1 ) # Generate a unique id for the node

        for node in self.get_connections( ):
            node.connections.add( self.uid )

        Node.nodes[ self.uid ] = self

    def get_connections( self ):
        connections = []
        for connection in self.connections:
            connections.append( Node.nodes[ connection ] )

        return connections

    def get_json( self ):
        return node_format \
        .replace( "{connections}", str( self.connections ) ) \
        .replace( "{name}", self.name ) \
        .replace( "{uid}", str( self.uid ) )
        # this is bad, like really really bad but it just converts to json

    def save( ):
        json = "[\n"
        for uid in Node.nodes.keys( ):
            json += Node.nodes[ uid ].get_json( )

        json += "]"

        return json

jag = Node( [], "Jag" ).uid
du  = Node( [jag], "Du" ).uid

print( Node.save( ) )


