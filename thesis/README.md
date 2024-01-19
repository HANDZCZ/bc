
# Bachelor's thesis

In this folder you can find my entire bachelor's thesis.
The thesis should comply with [UHK requirements](https://www.uhk.cz/cs/fakulta-informatiky-a-managementu/studium/statni-zaverecne-zkousky-a-kvalifikacni-prace/kvalifikacni-prace-bakalarska-diplomova)
([2023 version](https://www.uhk.cz/file/edee/fakulta-informatiky-a-managementu/uredni-deska/ridici-akty/vynosy-dekana/2023/metodicke-pokyny-pro-vypracovani-bakalarskych-a-diplomovych-praci.pdf))
for bachelor's thesis.

You can use these files for your own bachelor's thesis with some small or minimal changes.
In the filters folder are some modified filters and some filters made by me.
Chapters and pictures folder store all chapters and pictures respectively.

You will need to export your edited title page to pdf from the template provided to you.
It will most likely be provided to you in docx format, also don't forget to fix their date (if they didn't fix it), it should have spaces after dots.

## Running without docker

As of writing this readme pandoc and other tools already updated and broke things.
This thesis was written with pandoc 3.1.2, pandoc-crossref 0.3.16.0 and pandoc-plot 1.7.0.

Apart from needing to install pandoc, texlive (it's recommended to install most of the packages), and lualatex.
You will also need to install d2 for diagrams and some backend for it (I use tala) or use a different toolkit offered by pandoc-plot.
Next, you will need to install pygments for code highlighting (pygments are used by minted) and plotly for plots or some other toolkit supported by pandoc-plot.
You will also need to download missing fonts.

## With docker

If you want to use this as is, without adding new things then you can use my [docker image](https://hub.docker.com/repository/docker/handz/pandoc-toolkit/general).

You just need to mount your folder with md files, template and other stuff (in case of this repo it's the thesis folder) to ``/home/user/thesis``.
If you don't run it with your build script it will drop you into bash (in this case don't forget to run it with the flags ``-it``).

example usage - mount current directory and run conversion:
```bash
$ docker run -it --rm -v "$(pwd):/home/user/thesis" handz/pandoc-toolkit:3.1.2 "./create_thesis.sh"
```

