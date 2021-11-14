use crate::strs::{info, list, warn};

pub fn tut() {
    warn("First run detected; Showing Tutorial\n");
    info("Welcome to rpass! Here is a simple overview of how to use rpass to generate secure, memorable and reproducible passwords");
    info("Each rpass password has 4 components:");
    list("A prefix");
    list("A memorable secret");
    list("The name of the service you're generating the password for");
    list("A cipher\n");
    info("The prefix and cipher are randomly generated by rpass. The prefix is between 7 characters long, while the cipher is 512 characters (wow!)");
    info("Passwords are assembled by concatenating the prefix, the memorable secret and the service name mapped to the cipher");
    info("The memorable secret is the only value that is never stored or seen by rpass This is because the memorable secret is a phrase of your choice and the one and only thing you need to memorise for your passwords!");
    warn("Make sure to never share your memorable secret to anyone. This could be used by anyone with access to your device and user account to reverse engineer your passwords");
    info("This ensures that all passwords that are generated by rpass are:");
    list("Unique");
    list("Secure");
    list("Memorable");
    list("Reproducible");
    list("Never wholly stored anywhere on the machine\n");
    info("Knowing this, you can be sure your passwords are secure.");
    info("To run this tutorial again, run the command \"rpass --tut\" at any time!");
    warn("Make sure to scroll up and read the entire tutorial!");
}

pub fn help() {
    info(
        "rPass is a password manager that generates secure, memorable and reproducible passwords",
    );
    info("Usage:   rpass [options]");
    list("--tut:   Shows the tutorial (shows on first run)");
    list("--clean: Clears all cipher and prefix data \n             (your previous passwords will be unrecoverable)");
    list("--help:  Displays this message");
    info("Additionaly, the service name may be passed as an argument.");
}
