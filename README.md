
 ![Screenshot 2024-12-06 184420](https://github.com/user-attachments/assets/c05573bb-2f37-40f3-9389-966ff5941f23)


Here is an example of what paying chatgpt 20 dollars a month can make you without knowing how to code. It is an example of why php is just silly and outdated, and why you do not even have to rely on node js or golang to outshine php any more. This is made in rust, and rust is objectively the top dog. (It is FINE to make apps in php and similar as long as you deeply understand that they will never be secure or reliable like rust is.)

this is https://github.com/x1a7x/SIMPLE-RUST-TEXBOARD (which uses sled db) converted to use postgres. 

Seems to work very well, not tested fully yet tho. Rust IS the right lang, but apps still need to be tested and modified. Lightning fast even on tor because so far its only rust postgres and text. The sled db version is lightning fast, but under heavy load postgres would be best. Granted, not many websites need 2000 posts at the same time, and a lot of the modern solutions like sled db can handle 500 or more posts at once quite well and as fast as it gets. This app aims to be versatile- low amount of users to a high amount of users and still fast overall without being stopped. (given the same hardware as other boards). This Rust app wants to be ultra secure, ultra reliable, and unstoppable. Rust is currently the best choice for that. 

This is the early code to an imageboard that will be superior to existing ones. Feel free to correct me if im 
wrong, but i think https://usagi.reisen/ is the fastest well known ib that out there today (made in Golang) and this
aims to be far superior in speed and security. Feel free to make an issue and point out the very fastest well used imageboards.
(an untested app made for a single user does not count, it is not battle tested with multiple posters). This will end up being in the
top 3 most secure and fast imageboards of all. 
