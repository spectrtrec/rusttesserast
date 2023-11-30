FROM antonskh/rusttesserast:0.0.1

COPY . /

ENTRYPOINT ["tail", "-f", "/dev/null"]