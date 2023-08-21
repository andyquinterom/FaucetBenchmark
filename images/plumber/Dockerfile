FROM rocker/r-ubuntu:jammy
COPY api /api
WORKDIR /api
RUN apt-get update && apt-get install -y libcurl4-openssl-dev libssl-dev libxml2-dev libsodium-dev
RUN Rscript -e "renv::restore()"
EXPOSE 8000
CMD ["Rscript", "entrypoint.R"]
