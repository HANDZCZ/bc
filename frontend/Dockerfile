FROM nginx:alpine

# copy nginx config
WORKDIR /etc/nginx
COPY ./nginx.conf ./conf.d/default.conf

# copy frontend data
WORKDIR /frontend
COPY --chown=nginx:nginx ./dist/* ./

EXPOSE 80

ENTRYPOINT [ "nginx" ]
CMD [ "-g", "daemon off;" ]