
#  Trying to run a local LLM                               #
#  ------------------------------------------------------- #
# ollama in terminal to start the model                    #
# ollama pull llama3.1:8b localhost:11434                  #
# https://python.langchain.com/docs/how_to/local_llms/     #
# -------------------------------------------------------- #

from langchain_ollama import OllamaLLM

llm = OllamaLLM(model="llama3.1:8b")

llm.invoke("The first man on the moon was ...")