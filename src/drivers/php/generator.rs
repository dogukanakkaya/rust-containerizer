use super::composer::Composer;
use crate::drivers::js::package::Package;
use std::{fs::File, io::Write};

pub struct Generator { }

impl Generator {
    pub fn run(project_path: &String) {
        let mut dockerfile = File::create(format!("{}/Dockerfile", project_path)).expect("Dockerfile can't be created.");

        // used extensions can be found in composer.json's require object like "ext-pdo", "ext-mongo"
        let composer = Composer::new(format!("{}/composer.json", project_path), 2);

        let php_version = composer.data()["require"]["php"].as_str().unwrap_or_else(|| "latest")
        .replace("<", "")
        .replace(">", "")
        .replace("=", "")
        .replace("^", "")
        .replace("~", "");
        //let php_version = php_version.chars().filter(|x| !vec!['<', '>', '=', '^', '~'].contains(x)).collect::<String>();

        let mut dockerfile_contents = format!(
            "
FROM php:{}-fpm
WORKDIR /var/www/php
RUN apt-get update
            ", php_version);
        
        let package = Package::new(format!("{}/package.json", project_path));

        // install nodejs if package.json exists
        match package {
            Ok(p) => {
                dockerfile_contents.push_str("\nRUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -\nRUN apt-get-install -y nodejs");
            },
            _ => unimplemented!()
        }

                  
        dockerfile_contents = format!(
            "
RUN apt-get install -y \
    g++ \
    git \
    unzip \
    zlib1g-dev \
    libzip-dev \
    libpng-dev \
    libjpeg-dev \
    libicu-dev  \
    libonig-dev

RUN pecl install {}
RUN docker-php-ext-install {}
RUN docker-php-ext-enable {}

COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

COPY . .

RUN composer install
RUN npm i
            ",
            "pecl needed extensions",
            "other extensions",
            "enable needed extensions"
        );

        println!("Dockerfile: \n{}", dockerfile_contents);

        match dockerfile.write_all(dockerfile_contents.as_bytes()) {
            Ok(()) => unimplemented!(),
            Err(_) => unimplemented!()
        }
    }
}