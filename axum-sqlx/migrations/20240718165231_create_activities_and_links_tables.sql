CREATE TABLE activities (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    occurs_at TIMESTAMP NOT NULL,
    trip_id UUID NOT NULL,
    CONSTRAINT activities_trip_id_fkey FOREIGN KEY (trip_id) REFERENCES trips (id) ON DELETE RESTRICT ON UPDATE CASCADE
);

CREATE TABLE links (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    trip_id UUID NOT NULL,
    CONSTRAINT links_trip_id_fkey FOREIGN KEY (trip_id) REFERENCES trips (id) ON DELETE RESTRICT ON UPDATE CASCADE
);
