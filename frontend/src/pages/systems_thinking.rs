use leptos::*;

#[component]
pub fn SystemsThinkingPage() -> impl IntoView {
    view! {
        <div class="container mx-auto p-4 bg-gray-900 text-white font-sans">
            <h1 class="text-4xl font-bold text-center text-teal-300 mb-8">
                Apply Systems Thinking to Instructional Design
            </h1>

            <div class="bg-gray-800 bg-opacity-50 p-6 rounded-lg shadow-lg mb-8">
                <h2 class="text-2xl font-semibold text-pink-400 mb-4">Executive Summary</h2>
                <p class="text-lg text-gray-300">
                    This portfolio piece demonstrates the application of systems thinking to resolve a core pedagogical conflict in instructional design: the tension between the individualized focus of Cognitive Load Theory (CLT) and the social-constructivist principles of Vygotsky{"'"}s Sociocultural Theory. The resolution is achieved by introducing "Psychological Safety" as a mediating variable, creating a robust framework for designing learning environments that are both cognitively efficient and socially supportive.
                </p>
            </div>

            <div class="bg-gray-800 bg-opacity-50 p-6 rounded-lg shadow-lg">
                <h2 class="text-2xl font-semibold text-pink-400 mb-4">Reflection</h2>
                <div class="space-y-4 text-lg text-gray-300">
                    <p>
                        The primary challenge was to architect a learning system that respects the cognitive limits of an individual, as prescribed by CLT, without sacrificing the profound learning benefits of social interaction central to Vygotsky{"'"}s work. At first glance, these two theories appear to be at odds. CLT suggests minimizing extraneous cognitive load, which can include the complexities of social interaction. Conversely, Vygotsky argues that learning is fundamentally a social process, where learners co-construct knowledge. A purely CLT-focused design risks creating an isolated, sterile experience, while a purely Vygotskian one could overwhelm a novice learner.
                    </p>
                    <p>
                        The key insight was to reframe the problem through the lens of systems thinking, treating the learning environment as a dynamic system with interconnected elements. The tension between CLT and Vygotsky is not a paradox to be solved but a polarity to be managed. The introduction of Psychological Safety, a concept from organizational psychology, acts as the unifying bridge. By establishing an environment of high psychological safety, the cognitive load associated with social interaction (e.g., fear of judgment, social anxiety) is significantly reduced. This creates the necessary conditions for learners to engage in collaborative activities without overloading their working memory.
                    </p>
                    <p>
                        Therefore, the final design proposes a two-phase pedagogical model. The first phase, the "AI as a Mirror," provides a private, asynchronous space for learners to grapple with new concepts, minimizing cognitive load and building foundational knowledge. The second phase, "Social Scaffolding," is an opt-in, psychologically safe environment where learners can apply their knowledge in a social context, guided by mentors. This systems-based approach, mediated by psychological safety, allows both theories to coexist, creating a holistic and effective learning experience that is greater than the sum of its parts.
                    </p>
                </div>
            </div>
        </div>
    }
}
