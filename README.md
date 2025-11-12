
The Daydream Initiative: An Architectural Blueprint for a Next-Generation Instructional Design Platform


Section I: The Daydream Initiative as an Instructional Design Platform


1.1 Reframing the Mission: From Learner Tool to Creator's Sandbox

The initial proposal for the Daydream Initiative presented a compelling vision for transforming adolescent screen time into a productive journey of academic and personal growth.1 It identified a critical "Edutainment Gap" between engaging but unstructured AI entertainment platforms and effective but narratively shallow gamified learning applications.1 While this learner-centric mission remains the ultimate goal, a deeper analysis reveals that to effectively and sustainably bridge this gap, the platform's primary focus must be reoriented. The most potent strategy for empowering learners is to first empower the creators of their learning experiences: the instructional designers.
Therefore, this document reframes the Daydream Initiative not as a single educational game, but as a comprehensive authoring environment and research platform. Its mission is to provide instructional designers (IDs), educators, and learning scientists with a powerful, intuitive tool for creating, testing, and deploying deeply engaging, narrative-driven learning experiences. This positions Daydream as a "creator's sandbox" for the Purdue Learning Design and Technology (LDT) community and the broader instructional design field.1 By focusing on the creator, Daydream can become a force multiplier, enabling a new class of educational content that is both pedagogically sound and intrinsically motivating. The platform is architected to allow IDs to apply established learning theories and build real educational interventions without requiring specialized programming expertise, thereby serving as a practical testbed for concepts explored in courses such as Educational Video Game Design and Motivation and Instructional Design.1

1.2 The Instructional Designer as Modern Storyteller

The role of the instructional designer is undergoing a significant transformation. Historically viewed as developers of static courseware, IDs are increasingly becoming architects of dynamic, personalized learning journeys. This evolution demands tools that move beyond simple content presentation and facilitate the creation of experiences that engage the whole learner—addressing not just cognitive objectives but also the affective domain of emotions, values, and motivation.1
Daydream is conceived to be a tool for this modern instructional designer, the ID as a storyteller. It is built on the premise that narrative is one of the most fundamental and effective structures for human meaning-making. The platform provides the necessary components for IDs to weave pedagogical goals—such as vocabulary acquisition or critical thinking skill development—into the fabric of a compelling story.1 By leveraging the archetypal structure of Joseph Campbell's monomyth, "The Hero's Journey," Daydream offers a natural scaffold for constructivist learning, where learners actively build understanding through experience.1 The ID is no longer just presenting information; they are designing a world, crafting challenges, and creating a context in which knowledge is not merely acquired but applied in the service of a meaningful quest. This approach aligns directly with the learning preferences of modern students, who prioritize interactivity, autonomy, and relevance.1 Daydream empowers the ID to become the guide who designs the epic journey for the learner-protagonist.

Section II: The Daydream Authoring Environment: Empowering Instructional Designers

To fulfill its mission as a premier authoring tool, the Daydream platform must provide an experience that is both powerful for experts and accessible for novices. Its design philosophy is a deliberate synthesis of proven paradigms from the most effective authoring tools in the industry, integrating their core strengths into a unified, next-generation environment.

2.1 A Synthesis of Proven Authoring Paradigms

The Daydream authoring interface is designed to feel familiar to instructional designers by incorporating the best features of leading tools, while combining them in a novel way to specifically support AI-driven narrative learning.

2.1.1 Narrative Flexibility of Twine

At its core, Daydream is a tool for telling interactive, non-linear stories.2 To facilitate this, the primary authoring view will be a visual, node-based editor inspired by Twine. In this view, each story segment, or "passage," is represented as a distinct node on a canvas.3 Instructional designers can create new passages and connect them with arrows to represent choices and branching paths, providing a clear, high-level map of the entire narrative structure.3 This visual approach dramatically lowers the barrier to entry for creating complex, choice-based narratives, as it does not require IDs to manage story logic through code.4 They can focus on the narrative flow and pedagogical structure, seeing at a glance how a learner's decision at one point can lead to different challenges and outcomes later in the journey.

2.1.2 Complex Interactivity of Articulate Storyline 360

While Twine excels at narrative branching, creating more granular, object-level interactivity can be complex, often requiring coding.2 To address this, Daydream will incorporate a powerful, no-code interaction engine modeled on the "triggers" and "states" system from Articulate Storyline 360.5 This system allows IDs to define cause-and-effect relationships with simple, menu-based commands.
For example, an ID could create an interaction where:
Trigger: The learner drags the "Ancient Key" object onto the "Locked Chest" object.
Action: The state of the "Locked Chest" changes from 'Locked' to 'Open'.
Action: A new vocabulary word, "elucidate," is revealed, along with a clue.
This system empowers IDs to build sophisticated puzzles, dynamic character interactions, and conditional events without writing a single line of code. They can make objects respond directly to learner actions, change a character's expression based on a dialogue choice, or make critical items appear in the inventory only after a specific learning objective is met.5 This capacity for custom, object-level interaction moves beyond simple branching paths and enables the creation of truly dynamic and responsive learning environments.

2.1.3 Seamless Media Integration of Genially

Modern learning experiences are multi-modal. To prevent the narrative from being a wall of text, Daydream will feature a media integration system inspired by the ease of use of platforms like Genially.7 The authoring interface will include a dedicated media library where IDs can upload and manage images, audio files, and video clips. These assets can then be dragged and dropped directly into story passages. Furthermore, the platform will support the embedding of external web content via iframes, allowing IDs to seamlessly integrate resources like YouTube videos, interactive maps, or even other web-based learning activities directly into the Daydream experience.8 This functionality is crucial for enriching the narrative and catering to diverse learning preferences, transforming the text-based adventure into a vibrant multimedia world.

2.2 The Modular ITS Architecture for IDs

The power of the Daydream authoring environment is rooted in its underlying architecture, which is modeled on a classic Intelligent Tutoring System (ITS).1 This architecture is composed of four distinct but interconnected modules. For the instructional designer, this modularity provides unparalleled flexibility, allowing them to customize every aspect of the learning experience through an intuitive graphical user interface.

2.2.1 The Expert Module (Content Authoring)

The Expert Module contains the core knowledge and content of the learning experience.1 In Daydream, an ID interacts with this module through a series of structured, form-based editors that are designed for rapid content creation and adaptation.
Vocabulary Banks: An ID can create and manage multiple collections of vocabulary words. The interface would allow them to input a target word, its definition, an example sentence, and associated metadata (e.g., difficulty level). They could create a bank for "SAT Preparation," then in a new project, create another for "Medical Terminology" without altering the underlying story structure.1
Story Arcs: Using the node-based editor, the ID defines the key plot points, choices, and challenges of the narrative. This structure can be saved as a template. An ID could design a complete Hero's Journey arc and then "re-skin" it for a different subject by simply swapping the Vocabulary Bank and updating the narrative text, dramatically accelerating development time.1
Character Templates: A dedicated editor allows IDs to define character archetypes, their motivations, and their dialogue trees. These templates can be saved and reused across different stories, ensuring consistency and saving time.1

2.2.2 The Tutor Module (Pedagogical Strategy)

This module is the pedagogical brain of the system, containing the AI logic that guides the learning process.1 The authoring tool gives the ID direct control over this logic.
AI Persona Selection: The ID can choose from a library of pre-configured AI storyteller personas. For the main narrative, they might select a standard "Narrator." For the "Reflection Quests," they would select the "Contemplative Guide," a Socratic persona designed to ask probing questions.1 Researchers could even create and test new AI personas by swapping in different models.1
Instructional Strategy Configuration: The ID can set the rules for the AI tutor. They can define parameters such as the frequency of new vocabulary introductions, the conditions that trigger a "Reflection Quest" (e.g., after every major boss battle), and the type of feedback provided to the learner (e.g., immediate corrective feedback vs. delayed, summary feedback).1 This allows for fine-grained control over the pedagogical approach.

2.2.3 The Student Module (Assessment and Analytics)

This module tracks the learner's progress and state of knowledge.1 The Daydream authoring tool provides the ID with a powerful, customizable analytics dashboard to define and monitor learning.
Success Metrics: The ID can define what constitutes success for their specific learning objectives. For vocabulary, "mastery" could be defined as a learner correctly using a new word in context three times. For critical thinking, it might involve tracking the decision paths a learner takes through a complex ethical dilemma.1
Data Visualization: The dashboard will provide visualizations of learner data, showing common misconceptions, popular narrative paths, and progress towards learning goals. This rich data is invaluable for formative assessment, allowing educators to understand where learners are struggling and enabling IDs to iteratively improve their course design based on real user behavior.1

2.2.4 The User Interface Module (Presentation Layer)

This module governs the look and feel of the final learning experience.1 To ensure that each creation feels unique, the authoring tool will include a theme editor.
Customization: Through simple configuration files and a visual editor, IDs can change the color palette, select fonts, and adjust the layout of UI elements like the inventory and quest log.1 This allows them to align the presentation with their institution's branding or to create a specific mood and atmosphere that enhances the narrative (e.g., a dark, gothic theme for a mystery story).
The following table provides a comparative analysis of Daydream's proposed authoring paradigm against established tools, highlighting its unique synthesis of features.
Table 1: Comparison of Authoring Tool Paradigms

Feature
Articulate Storyline 360
Twine
Genially
Daydream (Proposed)
Narrative Structure
Primarily linear with branching capabilities 10
Inherently non-linear, node-based 2
Slide-based, primarily linear 7
Inherently non-linear, visual node-based editor
Interactivity Engine
Advanced triggers, states, and layers 5
Link-based; complex interactions require coding 2
Pre-built widgets and click-to-reveal animations 8
Storyline-inspired triggers and states for no-code complexity
Media Integration
Robust support for video, audio, images 6
Requires external hosting and HTML links 3
Seamless drag-and-drop and web embedding 8
Genially-inspired drag-and-drop library and web embedding
AI-Driven Pedagogy
None
None
AI for content generation (images, text) 7
Core feature: Modular AI Tutor for adaptive scaffolding & reflection
Assessment & Analytics
SCORM/xAPI compliant; robust quizzing 10
None natively; requires custom coding
Built-in analytics dashboard and live polling 11
Customizable analytics dashboard tied to learning objectives
Target Use Case
Corporate e-learning, interactive simulations 10
Interactive fiction, non-linear narratives 3
Interactive presentations, marketing materials 7
Narrative-driven intelligent tutoring and educational games
Technical Skill Required
Low to moderate
Low for basic stories; high for complex interactivity
Low
Low to moderate; no coding required


Section III: Advanced Technology Integration for Immersive Learning

To create truly compelling and effective learning experiences, the Daydream platform will integrate a suite of modern AI technologies. These tools will enhance immersion, improve accessibility, and provide instructional designers with a powerful creative palette for bringing their narratives to life.

3.1 Voice-Driven Interaction: The Power of Speech-to-Text (STT)

A significant limitation of traditional text-based adventures is the friction of typing. To create a more natural and immersive interface, Daydream will incorporate a speech-to-text (STT) engine, allowing learners to speak their commands and dialogue choices directly to the game. This not only enhances the feeling of a direct conversation with the narrative world but also serves as a critical accessibility feature.

3.1.1 Technical Implementation Strategy

The selection of an STT engine is a critical architectural decision with significant implications for cost, performance, and data privacy. Commercial APIs, such as Google Cloud Speech-to-Text or Rev AI, offer high accuracy, support for over 125 languages, and are relatively easy to integrate.12 However, they operate on a pay-per-use model, which would introduce ongoing operational costs—a significant challenge for a free, non-profit platform. More importantly, using a cloud-based API requires sending sensitive user data, in this case, student voice recordings, to a third-party server. This creates a substantial legal and ethical burden, particularly under strict data privacy laws like COPPA and GDPR.1
A superior approach for the Daydream project is to leverage a high-performance, open-source STT model. OpenAI's Whisper, for example, offers accuracy that is competitive with commercial solutions and is available under a permissive license.14 Other robust open-source toolkits like SpeechBrain also provide powerful alternatives.15 The key advantage of these models is that they can be hosted locally on the project's own servers (or even on-device in future iterations).16 This decision is not merely a financial one; it is a foundational choice for privacy. By processing all voice data within a secure, self-contained environment, the platform can avoid the complexities and risks of third-party data sharing, dramatically simplifying compliance with student privacy regulations. This makes the open-source path the only viable option for a project committed to being globally free and legally sound.

3.1.2 User Experience

To ensure a seamless interaction, the STT system will utilize real-time streaming transcription.12 As the learner speaks, their words will appear on the screen almost instantly, providing immediate feedback that their input is being received and understood. This creates a fluid conversational loop that minimizes latency and reinforces the sense of immersion, making the interaction feel as natural as speaking to another person.

3.2 Dynamic Visual Narratives: Generative AI for Imagery and Video

While the core of Daydream is text-based, visual elements can profoundly enhance narrative immersion and emotional impact. Generative AI provides a revolutionary means for instructional designers to create custom visual assets without needing artistic skills or a large budget.

3.2.1 Asset Generation Workflow

The Daydream authoring tool will integrate generative AI capabilities directly into the workflow. When an ID is writing a description of a new location, they could highlight the text, right-click, and select "Generate Scene Image." A text prompt would be automatically created from their description (e.g., "A photorealistic image of a mystical library with towering shelves and glowing runes"), and an AI image generator like Adobe Firefly would produce a unique visual to accompany the scene.18 Similarly, to set the mood for a new chapter, an ID could write a prompt like "A short, cinematic video of a storm gathering over a medieval castle" and use a text-to-video model like Luma AI's Dream Machine to generate an atmospheric establishing shot.19 This allows for the rapid creation of a visually rich and consistent world that is perfectly tailored to the narrative.

3.2.2 Leveraging Non-Profit Offerings

A key component of the platform's sustainability model is to aggressively leverage free and discounted software offerings for non-profits. Adobe, for instance, offers its Adobe Express Premium plan for free to qualified 501(c)(3) organizations.21 This plan includes generative credits for Adobe Firefly, providing a no-cost pathway for IDs to create high-quality images and video clips.18 By building the platform's operational plan around such programs, Daydream can provide powerful creative tools to its users without incurring significant licensing fees. Other platforms like Canva, HeyGen, and Colossyan also offer features and plans that can be explored for non-profit partnerships.22

3.2.3 AI-Powered Narration (Text-to-Speech)

To complete the multi-sensory experience, Daydream will integrate a high-quality text-to-speech (TTS) engine to provide voice narration for the story. This enhances accessibility for learners with reading difficulties and adds a layer of polish and immersion. While commercial solutions exist, the project will prioritize open-source models to maintain its commitment to cost-effectiveness and local data processing. Advanced open-source TTS systems like Fish-Speech (now OpenAudio) and Index-TTS offer features that go far beyond robotic speech.25 They support emotion and tone control through simple text tags, allowing an ID to mark dialogue as (whispering) or (shouting), and the AI voice will modulate its delivery accordingly.25 This level of expressive control allows the narration to dynamically match the emotional context of the story, providing a rich auditory experience that rivals human voice acting at a fraction of the cost.

3.3 Ethical AI in Media Generation: A Core Pedagogical Principle

The power of generative AI comes with significant ethical responsibilities, especially in an educational context. The Daydream platform must be designed with a proactive and principled approach to the ethical use of this technology.

3.3.1 Mitigating Bias and Harmful Stereotypes

Generative models trained on vast internet datasets can inadvertently reproduce and amplify societal biases, creating content that is stereotypical or harmful.1 There have been documented cases of NGOs using AI to generate "poverty porn"—stereotypical and racialized images of poverty—because it is cheaper and avoids the complexities of obtaining consent from real individuals.28 This practice is ethically fraught and runs counter to the goals of dignified storytelling. To combat this, the Daydream platform will implement several safeguards. The back-end prompts sent to the image generation models will be engineered to include modifiers that encourage diversity and discourage stereotypical depictions. Furthermore, the authoring tool will provide IDs with clear guidelines on ethical prompting and will integrate with ethically sourced stock photo libraries as an alternative to pure generation, giving creators a choice to use real, dignified photography when appropriate.

3.3.2 Ensuring Psychological Safety

The core pedagogical innovation of Daydream is the "AI as a Mirror" feature, which relies on creating a space of high psychological safety where learners feel comfortable engaging in self-reflection.1 The use of AI-generated visuals must support, not undermine, this goal. While some platforms specialize in creating AI avatars for training videos, this can be a double-edged sword.24 An AI-generated character that is perceived as judgmental, unrealistic, or non-inclusive could easily break the learner's trust and shut down the reflective process. Therefore, all generative AI features within Daydream will be governed by a principle of "do no harm." The visuals generated must be carefully curated and controlled to be inclusive, respectful, and supportive of a safe and non-judgmental learning environment.
The following table summarizes the technical and strategic trade-offs for the platform's STT integration.
Table 2: Speech-to-Text (STT) Integration Options

Solution
License/Cost Model
Hosting
Accuracy (WER)
Key Features
Privacy Implications
OpenAI Whisper
MIT (Permissive) / Free
Local / On-Premise
State-of-the-art, competitive with commercial 14
Multilingual, robust performance, multiple model sizes 14
Excellent. Voice data remains within the platform's secure infrastructure, simplifying compliance.
Google Cloud STT
Commercial / Pay-per-use
Cloud (Google)
High accuracy, industry standard 12
125+ languages, streaming transcription, model adaptation 12
High Risk. Requires sending student PII (voice data) to a third party, creating a significant COPPA/GDPR compliance burden.
Rev AI
Commercial / Pay-per-use
Cloud (Rev AI)
Very high accuracy, especially with human transcription option 13
Asynchronous & streaming, sentiment analysis, topic extraction 13
High Risk. Same as other cloud APIs; involves third-party processing of sensitive student data.
SpeechBrain
Apache 2.0 (Permissive) / Free
Local / On-Premise
Competitive, used in research and education 15
Holistic toolkit for various speech tasks, highly customizable 15
Excellent. Local hosting ensures data privacy and security, ideal for educational applications.


Section IV: Gamification and Motivational Design in a LitRPG Framework

To fully realize the platform's potential for engagement, Daydream will be architected to support the creation of narratives in the Gamelit and Literary Role-Playing Game (LitRPG) genres.48 This framework blends storytelling with explicit game mechanics, providing a powerful structure for integrating educational content, particularly mathematical concepts, and leveraging proven motivational models to drive learner engagement.50

4.1 Defining the Genre: LitRPG as an Educational Framework

LitRPG is a genre where the story incorporates explicit game-like rules and mechanics that are visible to both the reader and the protagonist.49 Core elements of LitRPG that can be leveraged for educational design include:
Explicit Progression: Characters have visible statistics (e.g., Strength, Intelligence, Agility), levels, and skills that improve over time through experience.49 This provides a tangible, quantifiable measure of growth that directly reflects the learner's journey and mastery of concepts.49
Quests and Rewards: The narrative is driven by quests—specific tasks with clear objectives and rewards.50 Quests serve as the primary vehicle for introducing learning content, while rewards (such as experience points, new skills, or items) provide immediate positive reinforcement.53
System-Based World: The world operates under a defined set of rules, often referred to as "the System".55 This rule-based environment is ideal for introducing and reinforcing logical and mathematical principles, as the very physics and logic of the world are built on them.56
By adopting this structure, Daydream moves beyond simple gamification (adding points or badges to a task) and into game-based learning, where the learning is intrinsically woven into the fabric of the game itself.58

4.2 The Progression System: A Scaffold for Learning and Motivation

The heart of any LitRPG is its progression system, which details how a character grows in power and ability.55 In Daydream, this system will be a core tool for instructional designers to structure and motivate learning.
The Power Arc and Progression Curve: An ID can design the "power arc"—the path of strength from novice to master—to mirror a curriculum's learning objectives.61 The "progression curve" dictates the pace of this journey, controlling how much experience is needed to advance.61 For mathematical concepts, this curve can itself be a mathematical function (e.g., linear, exponential), with the difficulty of quests and the experience rewards balanced to ensure a steady but challenging learning pace.61
Multi-Faceted Progression: Progression in Daydream will not be limited to combat. To create a richer learning environment, experience points (XP) can be awarded for a variety of activities that align with educational goals, such as:
Practice and Understanding: Gaining XP for successfully applying a new concept or practicing a skill.63
Innovation and Discovery: Earning significant rewards for solving a problem in a novel way or discovering a hidden connection between concepts.63
Crafting and Building: Applying knowledge to create something new within the game, such as using geometric principles to design a structure.49

4.3 Motivational Modeling through Game Mechanics

The LitRPG framework provides a natural environment for applying established motivational theories to instructional design. The goal is to create an experience that is not just educational but intrinsically rewarding.64
Self-Determination Theory (SDT): Daydream's mechanics will be designed to satisfy the three basic psychological needs identified by SDT 66:
Autonomy: The branching narrative and choice-based quests give learners a sense of control over their learning path and journey.66
Competence: The visible progression system—leveling up, gaining new skills, and overcoming challenges—provides constant, informative feedback that fosters a feeling of mastery and growth.66
Relatedness: The compelling narrative and interactions with in-game characters create a strong connection to the story and its world, making the learning context meaningful.66
Balancing Intrinsic and Extrinsic Motivation: While LitRPG mechanics like levels and loot are extrinsic rewards, they can be designed to support, rather than undermine, intrinsic motivation.51 Quests can be framed not as chores to be completed for a reward, but as meaningful challenges that drive the story forward and empower the learner.51 The "reward" becomes the feeling of competence and the narrative progress itself, with the XP and items serving as tangible markers of that achievement.58

4.4 Integrating Mathematical Concepts into the Core Loop

Mathematics is the native language of game design, governing everything from physics and graphics to the balance of the in-game economy.57 The Daydream platform will empower IDs to make this implicit mathematical foundation an explicit and engaging part of the learning experience.
Math as a Mechanic: Instead of presenting learners with abstract word problems, mathematical concepts can be integrated directly into the game's core mechanics and quests.59
An ID could design a quest requiring the learner to use trigonometry to aim a catapult and breach a castle wall.57
A crafting system could require the use of algebra to determine the correct ratios of ingredients needed to forge a magical item.57
A trade or resource management quest could involve statistical analysis to predict market fluctuations or optimize a supply chain.62
Conceptual Learning Through Play: By embedding math in this way, the platform moves beyond simple drill-and-practice and facilitates conceptual understanding.59 Learners are not just solving equations; they are using mathematical thinking as a tool to solve meaningful problems within the narrative world.68 This process of application, experimentation, and reflection within a game context helps build deeper and more flexible mathematical knowledge.68
The following table maps the proposed LitRPG mechanics to their corresponding motivational principles and potential for mathematical integration.
Table 3: Mapping LitRPG Mechanics to Educational and Motivational Goals

LitRPG Mechanic
Description
Motivational Principle (SDT)
Potential Math Integration
Stats & Levels
Quantifiable attributes (e.g., Logic, Creativity) and an overall level that represents accumulated experience.55
Competence: Provides clear, immediate feedback on progress and mastery.66
XP progression curves (algebraic functions), statistical analysis of character builds.62
Skills & Abilities
Specific capabilities that are unlocked and improved through practice and leveling up.49
Competence: Allows for specialization and tangible growth in specific areas of knowledge.66
Skill improvement rates (calculus), probability of success for skill-based actions.57
Quests
Narrative-driven tasks with clear objectives that guide the learner through the educational content.50
Autonomy: Offers choices in which quests to pursue and how to complete them.67
Embedding problems from geometry, algebra, or logic directly into quest objectives.52
Loot & Rewards
Tangible rewards (items, currency, titles) for completing quests and overcoming challenges.50
Extrinsic Reinforcement: Provides positive feedback and a sense of achievement that supports intrinsic goals.58
Game economy balancing, probability calculations for loot drops, optimization problems for gear sets.62


Section V: Engineering Pedagogy: AI-Driven Cognitive Load Management

A central thesis of the Daydream project is that effective instructional design must be grounded in the science of how people learn. The platform's architecture is therefore not just a technical framework but a pedagogical one, engineered to actively manage the learner's cognitive load. By applying the principles of Cognitive Load Theory (CLT), Daydream aims to optimize the learning experience, ensuring that the learner's limited mental resources are directed toward activities that foster deep understanding rather than being wasted on extraneous processing.

5.1 Foundations in Cognitive Load Theory (CLT)

Cognitive Load Theory, first proposed by John Sweller in the 1980s, provides a framework for designing instruction that is compatible with human cognitive architecture.30 The theory is based on the well-established premise that our working memory—the mental workspace where we actively process new information—is severely limited in both capacity and duration.30 It can only handle a few new pieces of information at once.33 If an instructional task overwhelms this limited capacity, learning is impaired or fails entirely.30 CLT posits that the total cognitive load experienced by a learner is composed of three distinct types 32:
Intrinsic Cognitive Load (ICL): This is the load imposed by the inherent complexity of the learning material itself.30 It is determined by the number of interacting elements that a learner must process simultaneously to understand a concept.32 Learning basic vocabulary has a low intrinsic load, while understanding a complex scientific theory has a high intrinsic load.
Extraneous Cognitive Load (ECL): This is an "ineffective" or "unhelpful" load that is not necessary for learning.30 It is imposed by poor instructional design, such as a confusing user interface, redundant information, or a split-attention effect where the learner must mentally integrate disconnected sources of information.32 The goal of good instructional design is to minimize this load.
Germane Cognitive Load (GCL): This is the "effective" load that is directly beneficial to learning.32 It refers to the mental effort a learner devotes to processing information deeply, constructing mental models (schemata), and integrating new knowledge with prior knowledge.32 The goal of good instructional design is to optimize this load.
The central challenge for any instructional system is to manage these three loads so that their sum does not exceed the learner's available working memory capacity.34

5.2 AI and Software Engineering to Manage Cognitive Load

The Daydream platform is engineered with specific features designed to dynamically manage each type of cognitive load, using AI to adapt the experience to the individual learner.

5.2.1 Managing Intrinsic Load

The intrinsic load of the material is not fixed; it is relative to the learner's expertise. The AI Tutor module in Daydream is designed to modulate this load through adaptive scaffolding and content segmentation.
Adaptive Scaffolding: When a learner encounters a new, complex problem within the narrative (a task with high "element interactivity," in CLT terms), the AI can provide a high degree of support.33 This might take the form of a "worked example," where the AI guides the learner step-by-step through the solution process.33 This initial support lowers the intrinsic load, allowing the learner to understand the process without being overwhelmed. As the learner demonstrates competence by solving similar problems, the AI dynamically fades this scaffolding, requiring more independent thought and gradually increasing the manageable intrinsic load.32
Content Segmentation: The Hero's Journey narrative structure provides a natural framework for segmenting the learning content into manageable chunks, or "quests".1 This is a key CLT strategy for managing complexity.32 The AI Tutor controls the pacing within these segments, introducing a limited number of new vocabulary words or concepts in each quest to ensure the learner is not required to process too many new elements at once.

5.2.2 Minimizing Extraneous Load

Extraneous load is the enemy of effective learning, and the Daydream platform is designed from the ground up to minimize it through careful UX design and adherence to multimedia learning principles.
Applying the Modality Effect: The modality effect suggests that presenting information in both auditory and visual formats can be more effective than a single format, as it utilizes separate channels in working memory.35 Daydream leverages this by combining AI-generated narration (auditory) with on-screen text and images (visual).32 This dual-channel presentation can enhance comprehension without increasing cognitive load.
Avoiding the Redundancy Effect: Conversely, presenting identical information in multiple formats simultaneously (e.g., reading on-screen text verbatim) is redundant and increases extraneous load.35 The Daydream platform is designed to avoid this. For example, when a character's dialogue is spoken by the TTS engine, the full text will not be simultaneously displayed on screen. Instead, a character portrait and name might appear, reducing redundant processing and freeing up cognitive resources.
Minimalist User Interface: The user interface is intentionally designed to be clean and focused. The "blank page" problem identified in early prototypes, which created uncertainty and thus extraneous load, is solved by persistent UI elements that always display the current quest objective, character status, and inventory.1 This provides a constant sense of direction and grounding, reducing the mental effort required to simply navigate the experience and allowing the learner to focus on the task at hand.36

5.2.3 Optimizing Germane Load

The ultimate goal of instruction is not just to present information, but to encourage the deep cognitive processing that leads to durable learning. Daydream is engineered to actively promote this germane load.
"AI as a Mirror" as a Germane Load Engine: The "Reflection Quests" are the platform's primary mechanism for inducing germane cognitive load.1 At the end of a major story arc, the AI shifts into a Socratic guide, prompting the learner with questions that require them to connect their in-game choices to their real-world values and thought processes.1 A prompt like, "The Oracle gave you a difficult choice. What personal belief influenced your decision?" forces the learner to engage in self-explanation and elaboration—two of the most powerful known techniques for promoting schema construction and deep learning.32 This is the "desirable difficulty" that constitutes true learning.
Situated Learning Context: By embedding all learning within a meaningful narrative context, Daydream encourages the creation of richer, more robust schemata.1 A vocabulary word like "precarious" is not learned as an isolated fact on a flashcard; it is learned and applied in the context of navigating a crumbling bridge to achieve a goal.1 This process of connecting new knowledge to a purpose and a rich context is a powerful driver of germane load, leading to more meaningful and transferable knowledge.
The platform's design represents a sophisticated synthesis of pedagogical theories. While the original proposal grounds Daydream in constructivism, where learners actively construct their own knowledge, CLT is sometimes criticized for promoting more direct, teacher-centered instruction.1 This creates an apparent theoretical tension. However, research indicates that constructivist approaches like problem-based learning are most effective when novices are given appropriate support to manage the initially high cognitive load.32 Daydream's architecture resolves this tension. The AI Tutor acts as an adaptive "Cognitive Load Governor." For a novice learner facing a new challenge, the AI can provide more explicit, direct instruction, reducing extraneous load and preventing cognitive overload. As the learner builds expertise and develops their own schemata, the AI can dynamically fade its support, opening up space for more open-ended, constructivist exploration. In this way, Daydream synthesizes the two theories, using CLT principles to create a safe and effective on-ramp to a rich constructivist learning environment.
The following table explicitly maps the platform's features to the principles of Cognitive Load Theory.
Table 4: Cognitive Load Management Strategies in Daydream

Cognitive Load Type
Design Principle
Corresponding Daydream Feature
Intended Learner Impact
Intrinsic (ICL)
Adaptive Scaffolding
AI Tutor provides "worked examples" for new, complex problems, then fades support as mastery increases.32
Manages inherent task difficulty, preventing overload for novices while challenging experts.
Intrinsic (ICL)
Content Segmentation
The Hero's Journey narrative structure breaks the learning journey into discrete, manageable "quests".1
Prevents overwhelming the learner by presenting complex material in smaller, sequential chunks.
Extraneous (ECL)
Modality Principle
Information is presented through multiple channels (e.g., AI narration + generated image).32
Leverages both visual and auditory processing channels in working memory to improve comprehension.
Extraneous (ECL)
Redundancy Principle
Avoids presenting identical information simultaneously (e.g., TTS narration is not accompanied by verbatim on-screen text).35
Eliminates unnecessary mental processing, freeing up cognitive resources for learning.
Extraneous (ECL)
Signaling & Coherence
A persistent, minimalist UI clearly displays the current quest objective, status, and inventory at all times.1
Reduces the mental effort required for navigation and orientation, allowing focus on the learning task.
Germane (GCL)
Self-Explanation & Elaboration
The "AI as a Mirror" feature uses Socratic questioning during "Reflection Quests" to prompt metacognition.1
Stimulates deep cognitive processing, forcing learners to connect actions to values and build robust mental models.
Germane (GCL)
Situated Cognition
Vocabulary and problem-solving skills are embedded within a meaningful, goal-oriented narrative context.1
Fosters the construction of rich, contextualized schemata, leading to more durable and transferable knowledge.


Section VI: A Blueprint for a Global, Open, and Sustainable Platform

To realize its full potential as a contribution to the educational community, the Daydream Initiative must be built on a foundation that is operationally sustainable, legally compliant on a global scale, and philosophically committed to open access. This requires a comprehensive blueprint that leverages existing non-profit infrastructure, adopts a privacy-first architectural design, and makes a strategic commitment to an open-source ethos.

6.1 Operational Model: Leveraging the Google for Nonprofits Suite

The proposal for Daydream to be run by "students of Purdue" necessitates a low-cost, highly efficient operational infrastructure.1 The Google for Nonprofits suite provides a comprehensive, no-cost solution that can serve as the project's entire operational backbone.37
Collaboration and Administration (Google Workspace): The student-led development and administration team will utilize the free Google Workspace for Nonprofits edition for all internal operations. This provides professional email addresses at a custom domain (e.g., @daydream.org), 100 TB of pooled cloud storage via Google Drive for all design documents, code repositories, and user feedback data, and collaboration tools like Google Docs, Sheets, and Slides.37 Google Meet will facilitate remote team meetings and collaboration with faculty advisors and external partners.38 This suite provides enterprise-grade tools at zero cost, making a student-run operation feasible and professional.
Hosting and Infrastructure (Google Cloud): The Daydream application itself—the web server, databases, and the self-hosted open-source AI models for STT and TTS—will be deployed on the Google Cloud Platform (GCP). As an eligible non-profit, the Daydream entity will apply for Google Cloud credits, which can significantly offset or entirely eliminate the costs of hosting a global-scale application.
Outreach and Community Building (Google Ad Grants & YouTube): To achieve the goal of having the "widest possible positive impact," the platform must reach its intended audience of educators, parents, and instructional designers.1 The Google Ad Grants program provides qualifying non-profits with up to $10,000 per month in in-kind search advertising credits.38 These funds will be used to run targeted campaigns to raise awareness and drive user adoption. The YouTube Nonprofit Program will be used to create a channel featuring tutorials on the authoring tool, case studies of its use in classrooms, and showcases of compelling learning experiences. Features like YouTube's "Link Anywhere cards" and donation buttons can be used to direct viewers to the platform and support the non-profit's mission.38

6.2 Global Legal Compliance: A Privacy-First Architecture

An educational application targeting adolescents must navigate a complex web of international laws designed to protect children's data. Daydream's architecture will be designed from the ground up to be "privacy-first," ensuring compliance with the most stringent regulations.
Adherence to COPPA (US) and GDPR-K (EU): The platform will be built to comply with the core principles of the Children's Online Privacy Protection Act (COPPA) in the United States and the child-specific provisions of the General Data Protection Regulation (GDPR) in the European Union.41 Under GDPR, the default age of consent for information society services is 16, though Member States may lower it to 13.42 To ensure global compliance, Daydream will set its age of consent to 16 and require verifiable parental consent for any user below that age.43
Verifiable Parental Consent (VPC): Before any data is collected from a user under 16, the platform will implement a robust VPC process. This will involve methods designed to ensure, with reasonable certainty, that the person providing consent is indeed the child's parent or legal guardian, as required by law.44
Data Minimization and Security: The platform will adhere to the principle of data minimization, collecting only the personal information that is strictly necessary for the educational function of the app.46 All user data, particularly personally identifiable information (PII) such as journal entries and voice recordings, will be encrypted both in transit and at rest.44 The architectural decision to use locally-hosted, open-source AI models is the cornerstone of this security strategy. By processing sensitive data like voice commands on its own servers, the platform avoids sharing PII with third-party AI providers, minimizing the legal and security risks inherent in such transfers.46
Transparent Privacy Policies: The platform's privacy policy will be written in clear, simple, and age-appropriate language.44 It will explicitly detail what data is collected, the purpose of its collection, how it is used, and with whom (if anyone) it is shared. It will also provide clear instructions for parents on how they can review, amend, or request the deletion of their child's personal information at any time, in accordance with their rights under COPPA and GDPR.45

6.3 Open Source Strategy: Maximizing Impact and Collaboration

The decision to release Daydream as an open-source project is fundamental to its mission as a "gift" to the educational community.1 The choice of license, however, is a critical strategic decision that will determine the nature of this gift and its future evolution.
Permissive Licenses (MIT, Apache 2.0): These licenses offer maximum freedom to developers. They allow anyone to use, modify, and redistribute the code for any purpose, including incorporating it into closed-source, proprietary commercial products, with only a requirement to include the original license and copyright notice.47 While this encourages the widest possible adoption and experimentation, it does not legally compel others to share their improvements back with the community.
Strong Copyleft License (GNU General Public License - GPLv3): A strong copyleft license like the GPLv3 also grants the freedom to use, modify, and redistribute the code. However, it includes a key condition: any derivative work that is distributed must also be released under the same GPLv3 license.47 This "share-alike" provision legally ensures that the project and all subsequent versions and adaptations remain free and open-source in perpetuity.

6.3.1 Recommendation

For the Daydream Initiative, the recommended license is the GNU General Public License, version 3 (GPLv3). The primary goal of this project is not to serve as a foundation for commercial products but to create a perpetually open and collaborative educational resource. The GPLv3 is the license that best protects this ethos. It ensures that any improvements made by the global community—whether by a university research lab, a high school coding club, or an individual instructional designer—must be shared back under the same terms. This creates a virtuous cycle of innovation within the open-source commons, perfectly aligning with the project's stated goal of having the widest possible positive impact on the world of education.1
The following table clarifies the strategic implications of this choice.
Table 5: Open Source License Comparison for Educational Impact

Consideration
MIT License
Apache License 2.0
GNU GPLv3
Freedom for Developers
Maximum freedom; can be used in proprietary software 47
High freedom; includes patent grant, good for enterprise use 47
High freedom, but derivative works must also be GPLv3 47
Obligation to Share Improvements
None. Improvements can be kept proprietary.47
None. Improvements can be kept proprietary.47
High. All distributed derivative works must be released under GPLv3.47
Protection Against Commercialization
Low. The code can be freely incorporated into closed-source, commercial products.
Low. The code can be freely incorporated into closed-source, commercial products.
High. Prevents the core project from being forked into a competing proprietary product.
Alignment with "Gift" Ethos
Moderate. The initial code is a gift, but its future is not protected.
Moderate. Similar to MIT, but with patent protection.
Excellent. Legally ensures the project and all its derivatives remain a permanent gift to the open-source community.
Recommendation for Daydream
Not Recommended
Not Recommended
Strongly Recommended


Section VII: Conclusion and Strategic Roadmap


7.1 Synthesizing the Vision

The Daydream Initiative, as reconceptualized in this document, represents more than an innovative educational application; it is a blueprint for a next-generation authoring platform. By uniquely integrating the narrative flexibility of tools like Twine, the interactive power of Articulate Storyline 360, and a modular architecture grounded in the principles of Intelligent Tutoring Systems, Daydream offers a powerful new sandbox for instructional designers. The integration of advanced AI—for voice interaction, generative visuals, and adaptive pedagogy—is not merely a technical enhancement but a core component of its pedagogical design. The platform's commitment to actively managing learner cognitive load through AI-driven scaffolding and reflection transforms it from a simple content delivery system into an engine for deep, meaningful learning.
This comprehensive vision, coupled with a sustainable operational model built on the Google for Nonprofits suite and a robust legal framework rooted in a privacy-first, open-source ethos, positions Daydream as a significant potential asset. It offers the Purdue Learning Design and Technology program a tangible platform for research, a practical tool for teaching the next generation of instructional designers, and a vehicle for making a lasting, positive contribution to the global educational landscape.

7.2 A Phased Implementation Roadmap

To translate this vision into a functional reality, a structured, phased approach to development is recommended. This roadmap prioritizes the development of core functionality before layering on more advanced features, ensuring a stable and iterative path to a full public launch.
Phase 1: The Authoring Core (The Clarity Update): The initial development phase will focus exclusively on building the core authoring environment for instructional designers. This includes implementing the visual, node-based narrative editor, the Storyline-inspired triggers and states system, and the user interfaces for the four-module ITS framework (Expert, Tutor, Student, UI). The goal of this phase is to create a functional, intuitive tool that allows IDs to build, save, and play through a basic, text-only interactive narrative. This phase directly addresses the "What do I do now?" problem by establishing a clear and usable authoring paradigm.1
Phase 2: AI Integration (The Value Update): With the core authoring tool in place, the second phase will focus on integrating the advanced AI technologies that define the platform's unique value. This involves deploying a self-hosted, open-source STT model (e.g., Whisper) for voice input and a high-quality, open-source TTS model (e.g., OpenAudio) for narration. Initial integration with a generative image model, leveraging non-profit offerings like the Adobe Express Premium plan, will also be implemented. This phase makes the educational value of the platform tangible to creators and learners alike.1
Phase 3: Pedagogical Refinement and Evaluation: This phase will see the full implementation and testing of the AI-driven cognitive load management algorithms within the Tutor Module. The "AI as a Mirror" reflection quests will be fully developed and integrated. Crucially, this phase will involve conducting formal pilot studies with students and faculty within the Purdue LDT program. These studies will gather both quantitative (learning analytics) and qualitative (user feedback) data to evaluate and refine the platform's pedagogical effectiveness and usability.
Phase 4: Global Launch and Community Building: In the final phase, the legal non-profit entity will be formally established. The mature, tested Daydream platform will be launched publicly under the chosen GPLv3 license, with its codebase made available in public repositories. The outreach strategy will be activated, utilizing Google Ad Grants and the YouTube Nonprofit Program to build a global community of users, contributors, and researchers around the platform, fulfilling its mission to serve as an open and evolving resource for the world of education.
Works cited
Purdue Daydream
Twine – Digital Humanities Toolkit - Sites at Gettysburg College, accessed October 25, 2025, https://dh.sites.gettysburg.edu/toolkit/tools/twine/
Twine | Art History Teaching Resources, accessed October 25, 2025, https://arthistoryteachingresources.org/2019/06/twine/
Twine: The Accessible Digital Tool - The Edith Lando Virtual Learning Centre - The University of British Columbia, accessed October 25, 2025, https://elvlc.educ.ubc.ca/2023/10/01/twine-2023-and-2024/
Storyline 360 All Features | Articulate, accessed October 25, 2025, https://www.articulate.com/360/storyline/all/
Know about Articulate Storyline 360, its Benefits and Features, accessed October 25, 2025, https://www.swiftelearningservices.com/know-about-articulate-storyline-360-its-benefits-and-features/
Genially | The easiest way to create interactive experiences, accessed October 25, 2025, https://genially.com/
Interactive content creator - Genially, accessed October 25, 2025, https://genially.com/features/interactive-content/
All Features- Explore the product | Genially, accessed October 25, 2025, https://genially.com/features
Exploring Storyline 360: Features, Functionality, and Pricing - Teachfloor, accessed October 25, 2025, https://www.teachfloor.com/elearning-glossary/storyline-360
All Features- Explore the product - Genially, accessed October 25, 2025, https://genially.com/features/
Speech-to-Text API: speech recognition and transcription - Google Cloud, accessed October 25, 2025, https://cloud.google.com/speech-to-text
Rev AI: Speech to Text API | Speech Recognition Service, accessed October 25, 2025, https://www.rev.ai/
openai/whisper: Robust Speech Recognition via Large-Scale Weak Supervision - GitHub, accessed October 25, 2025, https://github.com/openai/whisper
speechbrain/speechbrain: A PyTorch-based Speech Toolkit - GitHub, accessed October 25, 2025, https://github.com/speechbrain/speechbrain
DeepSpeech is an open source embedded (offline, on-device) speech-to-text engine which can run in real time on devices ranging from a Raspberry Pi 4 to high power GPU servers. - GitHub, accessed October 25, 2025, https://github.com/mozilla/DeepSpeech
speech-to-text · GitHub Topics, accessed October 25, 2025, https://github.com/topics/speech-to-text
Adobe Firefly - Free Generative AI for creatives, accessed October 25, 2025, https://www.adobe.com/products/firefly.html
AI Video Generator for Non-Profit Awareness Campaigns - Luma AI, accessed October 25, 2025, https://lumalabs.ai/create/ai-video-generator-for-non-profit-awareness-campaigns
Free AI Video Generator: Text to Video online - Adobe Firefly, accessed October 25, 2025, https://www.adobe.com/products/firefly/features/ai-video-generator.html
Adobe Express for Nonprofits, accessed October 25, 2025, https://www.adobe.com/nonprofits/express.html
Free AI Image Generator: Online Text to Image App - Canva, accessed October 25, 2025, https://www.canva.com/ai-image-generator/
Nonprofit Video Maker: Elevate Your Storytelling - HeyGen, accessed October 25, 2025, https://www.heygen.com/video/nonprofit-video-maker
Colossyan Creator - AI Video Generator, accessed October 25, 2025, https://www.colossyan.com/
fishaudio/fish-speech: SOTA Open Source TTS - GitHub, accessed October 25, 2025, https://github.com/fishaudio/fish-speech
index-tts/index-tts: An Industrial-Level Controllable and Efficient Zero-Shot Text-To-Speech System - GitHub, accessed October 25, 2025, https://github.com/index-tts/index-tts
Exploring the World of Open-Source Text-to-Speech Models - BentoML, accessed October 25, 2025, https://www.bentoml.com/blog/exploring-the-world-of-open-source-text-to-speech-models
AI-generated ‘poverty porn’ fake images being used by aid agencies, accessed October 25, 2025, https://www.theguardian.com/global-development/2025/oct/20/ai-generated-poverty-porn-fake-images-being-used-by-aid-agencies
The Top AI Video Generators for Nonprofits in 2025 | ReelMind, accessed October 25, 2025, https://reelmind.ai/blog/the-top-ai-video-generators-for-nonprofits-in-2025
Cognitive Load Theory and its Relation to Instructional Design: Perspectives of Some Algerian University Teachers of English - ERIC, accessed October 25, 2025, https://files.eric.ed.gov/fulltext/EJ1287466.pdf
Cognitive Load Theory Meets AI: Designing Better Learning Experiences - Mindsmith, accessed October 25, 2025, https://www.mindsmith.ai/blog/cognitive-load-theory-meets-ai-designing-better-learning-experiences
Challenging Cognitive Load Theory: The Role of Educational Neuroscience and Artificial Intelligence in Redefining Learning Efficacy - PMC - PubMed Central, accessed October 25, 2025, https://pmc.ncbi.nlm.nih.gov/articles/PMC11852728/
Cognitive Load Theory: How to Optimize Learning - Let's Go Learn, accessed October 25, 2025, https://www.letsgolearn.com/education-reform/cognitive-load-theory-how-to-optimize-learning/
Cognitive Load Estimation for Optimizing Learning within Intelligent Tutoring Systems, accessed October 25, 2025, https://www.researchgate.net/publication/221413775_Cognitive_Load_Estimation_for_Optimizing_Learning_within_Intelligent_Tutoring_Systems
Cognitive Load Theory: Learn Smarter, Not Harder - Saima AI, accessed October 25, 2025, https://saima.ai/blog/cognitive-load-theory
(PDF) Cognitive Load Theory: Implications for Instructional Design in Digital Classrooms, accessed October 25, 2025, https://www.researchgate.net/publication/390000832_Cognitive_Load_Theory_Implications_for_Instructional_Design_in_Digital_Classrooms
Google Workspace for Nonprofits edition, accessed October 25, 2025, https://support.google.com/a/answer/2858465?hl=en
Nonprofit Basics: The Google for Nonprofits Program - Double the Donation, accessed October 25, 2025, https://doublethedonation.com/google-for-nonprofits/
Google Workspace for Nonprofits: Collaboration Tools, accessed October 25, 2025, https://www.google.com/nonprofits/offerings/workspace/
Google for Nonprofits - TechSoup, accessed October 25, 2025, https://www.techsoup.org/google-for-nonprofits
Children's Online Privacy Protection Rule ("COPPA") - Federal Trade Commission, accessed October 25, 2025, https://www.ftc.gov/legal-library/browse/rules/childrens-online-privacy-protection-rule-coppa
Art. 8 GDPR – Conditions applicable to child's consent in relation to information society services, accessed October 25, 2025, https://gdpr-info.eu/art-8-gdpr/
What Are the Privacy Laws for Educational Apps That Collect Children's Data?, accessed October 25, 2025, https://thisisglance.com/learning-centre/what-are-the-privacy-laws-for-educational-apps-that-collect-childrens-data
App Privacy Requirements for Kids - iubenda help, accessed October 25, 2025, https://www.iubenda.com/en/help/114081-app-privacy-requirements-for-kids
Legal Requirements for Websites and Apps Used by Children - iubenda help, accessed October 25, 2025, https://www.iubenda.com/en/help/5717-legal-requirements-websites-apps-children
Data Privacy for Kids Apps: What Parents and Developers Need to Know - Countly, accessed October 25, 2025, https://countly.com/blog/data-privacy-kids-apps
Top Open Source Licenses Explained - Mend.io, accessed October 25, 2025, https://www.mend.io/blog/top-open-source-licenses-explained/
What is the difference between litRPG and gamelit? - Reddit, accessed October 25, 2025, https://www.reddit.com/r/litrpg/comments/1cug062/what_is_the_difference_between_litrpg_and_gamelit/
What Is LitRPG? Everything You Need to Know to Start Writing - Campfire, accessed October 25, 2025, https://www.campfirewriting.com/learn/litrpg
LitRPG: how it works and how to create a story - Amsel Suite, accessed October 25, 2025, https://www.amsel-suite.com/article/35/litrpg-how-it-works-and-how-to-create-a-story
How to Use Gamification in Your Classroom to Encourage Intrinsic Motivation - Waterford, accessed October 25, 2025, https://www.waterford.org/blog/gamification-in-the-classroom/
Turning Math Into a Game | Edutopia, accessed October 25, 2025, https://www.edutopia.org/article/turning-math-game/
What is LitRPG? A Complete Guide to Literary Role-Playing Games - Wizard's Respite, accessed October 25, 2025, https://wizardsrespite.com/2024/11/02/what-is-litrpg-a-complete-guide-to-literary-role-playing-games/
Quest systems in Role Playing Games - Ludogogy, accessed October 25, 2025, https://ludogogy.professorgame.com/quest-systems-in-role-playing-games/
The Typical Structures of Progression Based Power Systems (a guide) : r/ProgressionFantasy - Reddit, accessed October 25, 2025, https://www.reddit.com/r/ProgressionFantasy/comments/1de5nsn/the_typical_structures_of_progression_based_power/
What is LitRPG and why you should start reading it | by Gabriel Natucci - Medium, accessed October 25, 2025, https://gabrielnatucci.medium.com/what-is-litrpg-and-why-you-should-start-reading-writing-it-7d32c88d951f
Real-World Maths in Video Game Design: Integrating Magical Mathematical Concepts, accessed October 25, 2025, https://learningmole.com/maths-in-video-game-design/
Motivation and Learning Through Gamification - thinking pro, accessed October 25, 2025, https://www.thinkinghabitats.com/blog/motivation-and-learning-through-gamification
Integrating Game Mechanics and Math Content in Math Game Design - ProQuest, accessed October 25, 2025, https://search.proquest.com/openview/af30d7300df7870b173c236f75082a56/1?pq-origsite=gscholar&cbl=18750&diss=y
LitRPG & Progression Fantasy Series Starters | Aethon Books, accessed October 25, 2025, https://aethonbooks.com/2025/02/02/litrpg-progression-fantasy-series-starters/
The Basics of Power Arcs in LitRPG Magic Systems - C. R. Rowenson, accessed October 25, 2025, https://crrowenson.com/magic-systems/the-basics-of-power-arcs-in-litrpg-magic-systems/
How does Maths can be important in game design ? : r/gamedesign - Reddit, accessed October 25, 2025, https://www.reddit.com/r/gamedesign/comments/td5x3b/how_does_maths_can_be_important_in_game_design/
LitRPG Without Endless Killing – Looking for Books with Diverse Progression Systems : r/ProgressionFantasy - Reddit, accessed October 25, 2025, https://www.reddit.com/r/ProgressionFantasy/comments/1j4v3fd/litrpg_without_endless_killing_looking_for_books/
(PDF) Game, Motivation, and Effective Learning: An Integrated Model for Educational Game Design - ResearchGate, accessed October 25, 2025, https://www.researchgate.net/publication/221217604_Game_Motivation_and_Effective_Learning_An_Integrated_Model_for_Educational_Game_Design
Increasing Motivation and Maximizing Student Engagement: The Benefits of Gameful Learning | ASC Office of Distance Education - The Ohio State University, accessed October 25, 2025, https://ascode.osu.edu/increasing-motivation-and-maximizing-student-engagement-benefits-gameful-learning
Self-determination theory in Video Games: Misconceptions about Basic Psychological Needs | Nick Ballou, accessed October 25, 2025, https://nickballou.com/blog/sdt-in-video-games-basic-needs-misunderstandings/
Self-Determination Theory for Multiplayer Games - Digital Thriving Playbook, accessed October 25, 2025, https://digitalthrivingplaybook.org/big-idea/self-determination-theory-for-multiplayer-games/
Playing Games to Build Understanding - UNI ScholarWorks, accessed October 25, 2025, https://scholarworks.uni.edu/cgi/viewcontent.cgi?article=1003&context=mat_facpub
