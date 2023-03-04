//Structures In Rust - Music Library
//Max Edward | 3/3/23

#[derive(Debug, Clone, PartialEq)] //Derive macros/traits
                                   /*The Debug macro is used to enable printing a struct with the 
                                   println!("{:?}")
                                   statement.
                                   The Clone macro is used to create a new copy of a struct when it 
                                   is passed to a function.
                                   The PartialEq macro is used to compare two instances of a struct 
                                   and check if they are equal or not.*/
struct Song {
    //Constructor for Song
    title: String,  //title variable for name of song
    artist: String, //artist variable for name of artist
    length: i32,    // length variable for time of song (seconds)
}

#[derive(Debug, Clone, PartialEq)] //Derive macros/traits
struct Playlist {
    //Constructor for Playlist
    name: String,     //name of playlist variable
    songs: Vec<Song>, //song vector, for multiple songs in a playlist
}

#[derive(Debug)] //Derive macros/traits
struct MusicLibrary {
    //Constructor for MusicLibrary
    playlists: Vec<Playlist>, //playlist vector, for multiple playlists in the library
}

impl MusicLibrary {
    //Implementation of Music Library
    fn add_playlist(&mut self, name: String) {
        //Takes name and adds a playlist with given name to vector of MusicLibrary.
        self.playlists.push(Playlist {
            name,
            songs: Vec::new(),
        });
    }

    fn add_song_to_playlist(&mut self, playlist_name: &str, song: Song) -> Result<(), String> {
        //Takes a song and adds it to the playlist.
        //If a playlist with the entered name is found, returns OK. If not, then return an error.
        for playlist in &mut self.playlists {
            if playlist.name == playlist_name {
                playlist.songs.push(song);
                return Ok(());
            }
        }
        Err(format!("Playlist {} not found", playlist_name))
    }

    fn search_song_by_title(&self, title: &str) -> Vec<&Song> {
        //Takes a given title and searches for songs across all playlists with the title.
        //Returns a vector of the found results.
        let mut result = Vec::new();
        for playlist in &self.playlists {
            for song in &playlist.songs {
                if song.title.contains(title) {
                    result.push(song);
                }
            }
        }
        result
    }
}

fn main() {
    //Greet user
    println!("\nWelcome to the UAT Music Library!");

    //Creates an instance of MusicLibrary that can be used to store playlists, songs, and search for
    //songs in the library all while being able to change the constructor's contents.
    let mut music_library = MusicLibrary {
        playlists: Vec::new(),
    };

    //Loop for options, continues until user chooses exit.
    loop {
        println!("\nPlease select an option:");
        println!("1. Add playlist");
        println!("2. Add song to playlist");
        println!("3. Search for song by title");
        println!("4. Exit");

        //Sets choice equal to whatever the user inputs.
        let choice = get_user_input("\nEnter choice: ").parse::<u32>();

        //Match case statement for user choice
        match choice {
            //If 1, Take user input for playlist name, add it to the music_library, return playlist added.
            Ok(1) => {
                let playlist_name = get_user_input("\nEnter playlist name: ");
                music_library.add_playlist(playlist_name);
                println!("\nPlaylist added.");
            }
            //If 2, Take user input for playlist name to add to, song title, song artist, and song length.
            Ok(2) => {
                let playlist_name = get_user_input("\nEnter playlist name: ");
                let song_title = get_user_input("\nEnter song title: ");
                let song_artist = get_user_input("\nEnter song artist: ");
                let song_length = get_user_input("\nEnter song length (in seconds): ");

                //Create song object for storing title, artist, length.
                let song = Song {
                    title: song_title,
                    artist: song_artist,
                    length: song_length.parse().unwrap(),
                };

                //Call add song to playlist function, pass in playlist name and song.
                //Return song added to playlist if playlist found or no errors with song.
                match music_library.add_song_to_playlist(&playlist_name, song) {
                    Ok(_) => println!("\nSong added to playlist."),
                    Err(e) => println!("{}", e),
                }
            }

            //If 3, take user input for song name, call search song by title function, get vector
            //with all songs that contain the song title.
            //If no songs found, print No Songs Found. If songs found, list song title and artist.
            Ok(3) => {
                let search_title = get_user_input("\nEnter song title: ");
                let search_result = music_library.search_song_by_title(&search_title);

                if search_result.is_empty() {
                    println!("No songs found.");
                } else {
                    println!("\nSearch results:");
                    for song in search_result {
                        println!(
                            "Title: {}, Artist: {}",
                            song.title, song.artist
                        );
                    }
                }
            }

            //If 4, break out of loop, exit program.
            Ok(4) => break,
            _ => println!("\nInvalid choice."),
        }
    }
}

//Function for getting user input. 
//Import standard libraries, stdin, stdout, write.
fn get_user_input(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};

    //Prints prompt argument to console and flushes console.
    print!("{}", prompt);
    let _ = stdout().flush();

    //Create input string, remove white space.
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
