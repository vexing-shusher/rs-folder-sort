A simple script that takes in a path to a folder and sorts all of its contents into folders based on file extensions. For example:

- .txt, .doc, .docx, .odt files will go to the "Texts" folder;
- .jpg, .png, .tiff, .bmp files will go to the "Images" folder;
- .xls, .xlsx, .csv, .odt files will go to the "Tables" folder;
- .ppt, pptx, .odp files will go to the "Presentations" folder;
- .mp4, .mov, .avi, .wmv files will go to the "Videos" folder;
- .wav, .mp3, .flac files will go to the "Audios" folder;
- any files with extensions not listed above will go to their respective EXT folders, i.e. .pdf files will be stored in a PDF folder.

Treat your OCD :) And be careful with moving around system files: atm the script only ignores .ini files on Windows.
