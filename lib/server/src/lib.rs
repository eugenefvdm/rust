pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn store(
        name: String,
        host: String,
        ip_address: String,
        port: String,
        username: String,
        password: String
    ) {
    let connection = Connection::open("networks.db").unwrap();
    
    let query = format!("
        CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY, 
            name TEXT NOT NULL,
            host TEXT,
            ip_address TEXT,
            port TEXT,
            username TEXT,
            password TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        INSERT INTO servers ('name','host', 'ip_address', 'port', 'username',) VALUES ('{:?}','{:?}','{:?}','{:?}','{:?});
        ", command, output);
                
    connection.execute(query).unwrap();    
}

