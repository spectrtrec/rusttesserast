FROM rust:1.73

ARG TAG=5.3.3

RUN mkdir /tmp/tesseract && \
    echo "Update & upgrade" && \
    apt-get -y update && \
    apt-get -y upgrade && \
    apt-get install -y poppler-utils && \
    apt-get install -y libclang-dev && \
    echo "Install building tools" && \
    apt-get -y install autoconf-archive automake g++ libtool libleptonica-dev pkg-config tesseract-ocr-rus && \
    echo "Download leptonica" && \
    wget http://www.leptonica.org/source/leptonica-1.79.0.tar.gz  && \
    tar xvf leptonica-1.79.0.tar.gz && \
    echo "Compile leptonica" && \
    cd leptonica-1.79.0 && \
    ./configure && \
    make && \
    make install && \
    cd - && \
    echo "Clone tesseract 5 source" && \
    git clone https://github.com/tesseract-ocr/tesseract.git tesseract-ocr && \
    cd tesseract-ocr/ && \
    git checkout tags/${TAG} && \
    echo "Compile tesseract" && \
    ./autogen.sh && \
    ./configure && \
    make && \
    make install && \
    ldconfig && \
    cd /tmp && \
    rm -rf /tmp/tesseract && \
    cd /usr/local/share/tessdata && \
    wget https://github.com/tesseract-ocr/tessdata/raw/4.00/eng.traineddata && \
    wget https://github.com/tesseract-ocr/tessdata/raw/4.00/rus.traineddata && \
    export TESSDATA_PREFIX=/usr/local/share/tessdata
