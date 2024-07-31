CREATE TABLE participants (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT,
    email TEXT NOT NULL,
    is_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    is_owner BOOLEAN NOT NULL DEFAULT FALSE,
    trip_id UUID NOT NULL,

    CONSTRAINT participants_trip_id_fkey FOREIGN KEY (trip_id) REFERENCES trips(id) ON DELETE RESTRICT ON UPDATE CASCADE
);
