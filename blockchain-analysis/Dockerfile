FROM python:3.12-slim

WORKDIR /blockchain-analysis

COPY . /blockchain-analysis

RUN apt update
RUN pip install --upgrade -r requirements.txt

EXPOSE 8080

CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8080"]