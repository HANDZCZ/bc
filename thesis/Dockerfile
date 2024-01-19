FROM archlinux:base-devel-20240101.0.204074

# sync package databases
RUN pacman -Sy

# setupt user account
RUN useradd -Um -s /bin/bash user
RUN sed -i 's/# %wheel ALL=(ALL:ALL) NOPASSWD: ALL/%wheel ALL=(ALL:ALL) NOPASSWD: ALL/g' /etc/sudoers
RUN usermod -aG wheel user

# copy and install old pandoc packages
COPY utils/packages /root/packages
WORKDIR /root/packages
RUN pacman -U --noconfirm pandoc-bin-3.1.2-1-x86_64.pkg.tar.zst pandoc-crossref-bin-0.3.16.0-1-x86_64.pkg.tar.zst pandoc-plot-bin-1.7.0-0-x86_64.pkg.tar.zst

# switch to user
USER user
WORKDIR /home/user

# install yay
RUN sudo pacman -Sy --noconfirm --needed git base-devel
RUN git clone https://aur.archlinux.org/yay-bin.git && cd yay-bin && makepkg -si --noconfirm && cd .. && sudo rm -r yay-bin

# install packages
RUN yay -Sy --noconfirm d2 aur/tala texlive-basic texlive-bin texlive-fontsrecommended texlive-langczechslovak texlive-langfrench texlive-latex texlive-latexextra texlive-latexrecommended texlive-luatex texlive-pictures aur/python-plotly python-pygments aur/python-kaleido-bin aur/playwright chromium

# intit d2 cache
RUN echo 'x -> y' > input.d2
RUN d2 --pad 0 --layout tala input.d2 out.png
RUN rm input.d2 out.png

# install pygments theme
RUN git clone https://github.com/mozmorris/tomorrow-pygments.git
RUN sudo cp tomorrow-pygments/styles/* /usr/lib/python*/site-packages/pygments/styles/
RUN sudo rm -r tomorrow-pygments

# install fonts
COPY --chown=user:user utils/fonts .local/share/fonts

# create folder for documents and set it as working directory
RUN mkdir thesis
WORKDIR /home/user/thesis

# frop user in bash
CMD [ "bash" ]
