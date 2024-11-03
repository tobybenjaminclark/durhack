

import json
from openai import OpenAI
import openai
from boogle_python.speech.tts import TTS
import threading
import time


class PromptGenerator():
    def __init__(self, queue=None, parent=None):
        self.counter = -1
        self.interview_length = 3
        self.queue = queue
        self.tts = TTS(queue)
        self.parent = parent

        
        with open('boogle_python/key.txt', 'r') as file:
            key = file.read().strip()

        self.client = OpenAI(api_key=key)

    def shorten_description(self, inp:str) -> str:
        prompt = (
            f"shorten this description to one sentence: {inp}"
        )

        response = self.client.chat.completions.create(
                model="gpt-4",
                messages=[{"role": "assistant", "content": prompt}]
            )

        return response.choices[0].message.content

    
    def generate_prompt(self, inp: str) -> str:

        self.counter +=1 

        if self.counter == 0:

            self.log = []

            # Formulate the initial prompt with job description details
            prompt = (
                f"YOU ARE THE INTERVIEWER, ACT AS THE INTERVIEWER. Begin an interview with the candidate for the following position:\n\n{inp}\n\n"
                "Start with a greeting and an initial interview question. Make your response SHORT AND CONCISE."
            )
            
            response = self.client.chat.completions.create(
                model="gpt-4",
                messages=self.context + [{"role": "assistant", "content": prompt}]
            )

            print(response.choices[0].message)
            initial_message = response.choices[0].message.content
            self.context.append({"role": "assistant", "content": initial_message})
            reply = {
                "from": "interviewer",
                "message_data": initial_message
            }

            


        elif self.counter < self.interview_length:

            self.log.append({"from":"candidate", "message_data":inp})
            
            # Add user response to context
            self.context.append({"role": "user", "content": inp})
            
            # Generate the follow-up question
            response = self.client.chat.completions.create(
                model="gpt-4",
                messages=self.context
            )

            prompt = (
                f"YOU ARE STILL THE INTERVIEWER. The candidate has replied '{inp}'. How do you respond? Keep your response SHORT AND CONCISE and ask a question."
            )
            
            follow_up_question = response.choices[0].message.content
            self.context.append({"role": "assistant", "content": follow_up_question})
            reply = {
                "from": "interviewer",
                "message_data": follow_up_question
            }
        
        else:
            reply = {
                "from": "end_interviewer",
                "message_data": "Thank you!"
            }
            good_feedback, bad_feedback = self.handle_log()
            self.parent.stream_instances["emotion_stream"]["instance"].get_interview_data(good_feedback, bad_feedback)
            
            

        thread = threading.Thread(target=self.tts.speak, args=(reply["message_data"],))
        thread.start()
        

        self.log.append(reply)
        
        time.sleep(2)
        self.send_data(reply)

    def handle_log(self):
        print(f"log: {self.log}")


        
        # Generate a review of the user's responses
        good_review_prompt = (
            "Here are the user's responses during the interview:\n" +
            "\n" + "".join(["from: " + l["from"] + "\n" + " : " + l["message_data"] + "\n\n" for l in self.log]) +
            "\n\nDiscuss the strengths of this application in a concise paragraph."
        )

        bad_review_prompt = (
            "Here are the user's responses during the interview:\n" +
            "\n" + "".join(["from: " + l["from"] + "\n" + " : " + l["message_data"] + "\n\n" for l in self.log]) +
            "\n\nDiscuss the weaknesses of this application in a concise paragraph."
        )

        # Create a feedback response using the OpenAI API
        good_feedback_response = self.client.chat.completions.create(
            model="gpt-4",
            messages=[
                {"role": "system", "content": "You are a career coach."},
                {"role": "user", "content": good_review_prompt}
            ]
        )
        
        bad_feedback_response = self.client.chat.completions.create(
            model="gpt-4",
            messages=[
                {"role": "system", "content": "You are a career coach."},
                {"role": "user", "content": bad_review_prompt}
            ]
        )

        good_feedback = good_feedback_response.choices[0].message.content
        bad_feedback = bad_feedback_response.choices[0].message.content

        self.log = []
        return good_feedback, bad_feedback

        
    def send_data(self, data) -> None:
        data_json = json.dumps(data)
        self.queue.put(data_json)
        
    def start_interview(self, file_location: str):
        # read the file contents (interview)
        try:
            with open(file_location, "r") as f:
                contents = f.read()
        except:
            print("Failed to load actual interview, loading the other one.")
            with open("game/datafiles/Programmer_31.json", "r") as f:
                contents = f.read()

            # Initialize context with system prompt
        self.context = [
            {"role": "system", "content": f"Keep your reply short. Format your response. The candidate has applied for a position with the following details: {contents}. You are interviewing them for the role."}
        ]

        self.generate_prompt(contents)
        
    def is_leetcode_correct(self, title, question, examples, answer):
        prompt = (f"here is a leetcode question: {title} {question} {examples}. here is the answer in python: {answer}. reply YES if correct and NO if incorrect. only reply YES or NO")
        context = [{"role": "user", "content": prompt}]
        # Generate the follow-up question
        response = self.client.chat.completions.create(
                model="gpt-4",
                messages=context
            )
        
        reply = response.choices[0].message.content
        print(f"\n\n\n reply: {reply}")
        if "YES" in reply:
            return 1
        else:
            return 0
        


        
if __name__ == "__main__":
    prompt_generator = PromptGenerator()
    prompt_generator.generate_prompt("what a lovely day")
