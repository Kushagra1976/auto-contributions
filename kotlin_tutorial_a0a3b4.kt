// Objective: Learn to build a simple, rule-based conversational AI in Kotlin that "learns" by storing
// user inputs as data. We'll use Kotlin's powerful data classes and functional programming concepts
// to manage conversation state and responses efficiently.

// This AI will be very basic. It won't understand nuance or complex grammar. Instead, it will
// respond based on pre-defined rules and store user inputs to make future responses
// slightly more personalized (simulating learning).

// Let's start by defining the core structure of our conversation.
// A "Thought" represents a single turn in the conversation, from the AI's perspective.
// It holds the AI's current understanding (its knowledge base) and the response it generates.

data class ConversationState(
    val knowledgeBase: Map<String, String> = emptyMap()
)

// The 'knowledgeBase' is a map where keys are user inputs (or patterns)
// and values are the AI's pre-programmed responses to those inputs.
// Initially, it's empty, meaning the AI knows nothing.

// Now, let's define a function that represents the AI's "brain".
// This function takes the current conversation state and the user's input,
// and returns a new conversation state and the AI's response.

fun processUserInput(currentState: ConversationState, userInput: String): Pair<ConversationState, String> {

    // First, we'll create a variable to hold the AI's potential response.
    // We'll try to find a match in the existing knowledge base.
    var response = currentState.knowledgeBase[userInput]

    // If no direct match is found, we'll check for a general "fallback" response.
    // This simulates the AI's inability to understand something new.
    if (response == null) {
        response = "I'm not sure I understand. Could you rephrase that?"
    }

    // Here's where the "learning" happens (in a very simple way).
    // We'll create a *new* knowledge base by adding the user's input and the AI's response.
    // This is a functional approach: we don't mutate the original state, we create a new one.
    // This immutability makes code easier to reason about.
    val newKnowledgeBase = currentState.knowledgeBase.toMutableMap()
    newKnowledgeBase[userInput] = response // Store the user input and the corresponding response

    // We return the new conversation state and the generated response.
    return Pair(
        ConversationState(newKnowledgeBase.toMap()), // Convert back to immutable map for the new state
        response
    )
}

// Let's simulate a conversation.
fun main() {
    // Initialize the AI with an empty conversation state.
    var currentConversationState = ConversationState()

    println("AI: Hello! What can I help you with today?")

    // Simulate a few turns of conversation.
    val input1 = "What is your name?"
    println("User: $input1")
    val (state1, response1) = processUserInput(currentConversationState, input1)
    println("AI: $response1")
    currentConversationState = state1 // Update the AI's state for the next turn

    val input2 = "Tell me a joke."
    println("User: $input2")
    val (state2, response2) = processUserInput(currentConversationState, input2)
    println("AI: $response2")
    currentConversationState = state2

    // Now, let's see if the AI "remembers" our previous input.
    // This time, we'll provide a response that the AI has already "learned".
    val input3 = "What is your name?" // This was asked before
    println("User: $input3")
    val (state3, response3) = processUserInput(currentConversationState, input3)
    println("AI: $response3") // Notice the AI might give a different response if we programmed it to.
    currentConversationState = state3

    // In this *specific* example, the AI's logic for "What is your name?"
    // is hardcoded to "I am a simple AI." and it doesn't change its programmed response
    // based on *its own previous output*. It only adds the *user's input* and the *response it generated*
    // to its knowledge base. For a more advanced AI, you'd need more complex rules.

    // Let's add a rule to the AI *programmatically* to demonstrate how it could be expanded.
    // A real AI would have these rules defined upfront.
    // We'll override the previous response for "What is your name?" with a more specific one.
    // This is a simplified way to think about it.

    // In a real scenario, this mapping would be part of the AI's initial setup.
    // For this tutorial, we'll simulate adding a rule.
    // The 'processUserInput' function itself doesn't *decide* the response based on context,
    // but it *records* what it said.
    // To make the AI *actually* learn and *change* its response to a known input,
    // we'd need to modify the 'processUserInput' logic or have a richer state.

    // For this basic example, let's refine the AI's *initial* knowledge.
    // We'll start with some basic knowledge.
    println("\n--- Resetting AI with initial knowledge ---")
    var smarterConversationState = ConversationState(
        mapOf(
            "What is your name?" to "I am a rule-based AI.",
            "Tell me a joke." to "Why don't scientists trust atoms? Because they make up everything!",
            "How are you?" to "I am a program, so I don't have feelings, but I'm ready to help!"
        )
    )

    println("AI: Hello again! What's on your mind?")

    val input4 = "What is your name?"
    println("User: $input4")
    val (state4, response4) = processUserInput(smarterConversationState, input4)
    println("AI: $response4")
    smarterConversationState = state4

    val input5 = "How are you?"
    println("User: $input5")
    val (state5, response5) = processUserInput(smarterConversationState, input5)
    println("AI: $response5")
    smarterConversationState = state5

    val input6 = "What is your purpose?" // This is new
    println("User: $input6")
    val (state6, response6) = processUserInput(smarterConversationState, input6)
    println("AI: $response6") // This will be the fallback message
    smarterConversationState = state6

    // Notice how the AI's knowledge base grew after each interaction,
    // even though the *response logic itself* didn't change dynamically in this basic example.
    // The core learning here is the *storage* of interaction history.
}