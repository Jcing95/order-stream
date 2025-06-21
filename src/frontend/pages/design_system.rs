use leptos::prelude::*;

use crate::frontend::design_system::{
    Button, Input, Text, Icon, ThemeSwitcher, Card, Alert, Select, Spinner,
    theme::{Size, Intent, ComponentState},
    atoms::{InputType, TextVariant, FontWeight, IconVariant, CardVariant, SelectOption, SpinnerVariant},
};

#[component]
pub fn DesignSystemPage() -> impl IntoView {
    let input_value = RwSignal::new(String::new());
    
    view! {
            <div class="max-w-6xl mx-auto p-8 space-y-12">
                // Header with Theme Switcher
                <div class="flex items-start justify-between">
                    <div class="text-center flex-1">
                        <Text 
                            variant=TextVariant::Heading 
                            size=Size::Xl 
                            weight=FontWeight::Bold
                        >
                            "Order Stream Design System"
                        </Text>
                        <Text 
                            variant=TextVariant::Body 
                            intent=Intent::Secondary 
                            class="mt-2"
                        >
                            "Atomic components following the design system principles"
                        </Text>
                    </div>
                    <div class="flex flex-col items-end gap-2">
                        <Text variant=TextVariant::Label size=Size::Sm>
                            "Theme"
                        </Text>
                        <ThemeSwitcher size=Size::Md show_label=true />
                    </div>
                </div>

                // Button Section
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Buttons"
                    </Text>
                    
                    // Button Sizes
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Sizes"
                        </Text>
                        <div class="flex items-end gap-4 flex-wrap">
                            <Button size=Size::Xs>"Extra Small"</Button>
                            <Button size=Size::Sm>"Small"</Button>
                            <Button size=Size::Md>"Medium"</Button>
                            <Button size=Size::Lg>"Large"</Button>
                            <Button size=Size::Xl>"Extra Large"</Button>
                        </div>
                    </div>

                    // Button Intents
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Intents"
                        </Text>
                        <div class="flex gap-4 flex-wrap">
                            <Button intent=Intent::Primary>"Primary"</Button>
                            <Button intent=Intent::Secondary>"Secondary"</Button>
                            <Button intent=Intent::Success>"Success"</Button>
                            <Button intent=Intent::Danger>"Danger"</Button>
                            <Button intent=Intent::Warning>"Warning"</Button>
                            <Button intent=Intent::Info>"Info"</Button>
                        </div>
                    </div>

                    // Button States
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "States"
                        </Text>
                        <div class="flex gap-4 flex-wrap">
                            <Button state=ComponentState::Enabled>"Enabled"</Button>
                            <Button state=ComponentState::Disabled>"Disabled"</Button>
                            <Button state=ComponentState::Loading>"Loading"</Button>
                        </div>
                    </div>
                </section>

                // Input Section
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Inputs"
                    </Text>
                    
                    // Input Sizes
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Sizes"
                        </Text>
                        <div class="space-y-3">
                            <Input size=Size::Xs placeholder="Extra Small Input" />
                            <Input size=Size::Sm placeholder="Small Input" />
                            <Input size=Size::Md placeholder="Medium Input" />
                            <Input size=Size::Lg placeholder="Large Input" />
                            <Input size=Size::Xl placeholder="Extra Large Input" />
                        </div>
                    </div>

                    // Input Types
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Types"
                        </Text>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                            <Input input_type=InputType::Text placeholder="Text Input" />
                            <Input input_type=InputType::Email placeholder="Email Input" />
                            <Input input_type=InputType::Password placeholder="Password Input" />
                            <Input input_type=InputType::Number placeholder="Number Input" />
                            <Input input_type=InputType::Tel placeholder="Phone Input" />
                            <Input input_type=InputType::Search placeholder="Search Input" />
                        </div>
                    </div>

                    // Input Intents
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Intents"
                        </Text>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                            <Input intent=Intent::Primary placeholder="Primary" />
                            <Input intent=Intent::Secondary placeholder="Secondary" />
                            <Input intent=Intent::Success placeholder="Success" />
                            <Input intent=Intent::Danger placeholder="Error" />
                            <Input intent=Intent::Warning placeholder="Warning" />
                            <Input intent=Intent::Info placeholder="Info" />
                        </div>
                    </div>

                    // Input States
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "States"
                        </Text>
                        <div class="space-y-3">
                            <Input state=ComponentState::Enabled placeholder="Enabled" />
                            <Input state=ComponentState::Disabled placeholder="Disabled" />
                        </div>
                    </div>

                    // Interactive Input
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Interactive Example"
                        </Text>
                        <div class="space-y-2">
                            <Input 
                                placeholder="Type something..."
                                value=input_value
                            />
                            <Text variant=TextVariant::Caption>
                                "You typed: " {move || input_value.get()}
                            </Text>
                        </div>
                    </div>
                </section>

                // Text Section
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Typography"
                    </Text>
                    
                    // Text Variants
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Variants"
                        </Text>
                        <div class="space-y-2">
                            <Text variant=TextVariant::Heading size=Size::Lg>
                                "This is a Heading"
                            </Text>
                            <Text variant=TextVariant::Body>
                                "This is body text that provides information and context."
                            </Text>
                            <Text variant=TextVariant::Label>
                                "This is a label"
                            </Text>
                            <Text variant=TextVariant::Caption>
                                "This is caption text for additional details"
                            </Text>
                            <Text variant=TextVariant::Code>
                                "const code = 'example';"
                            </Text>
                        </div>
                    </div>

                    // Text Sizes
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Sizes"
                        </Text>
                        <div class="space-y-2">
                            <Text size=Size::Xs>"Extra Small Text"</Text>
                            <Text size=Size::Sm>"Small Text"</Text>
                            <Text size=Size::Md>"Medium Text"</Text>
                            <Text size=Size::Lg>"Large Text"</Text>
                            <Text size=Size::Xl>"Extra Large Text"</Text>
                        </div>
                    </div>

                    // Text Intents
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Intents"
                        </Text>
                        <div class="space-y-2">
                            <Text intent=Intent::Primary>"Primary text"</Text>
                            <Text intent=Intent::Secondary>"Secondary text"</Text>
                            <Text intent=Intent::Success>"Success text"</Text>
                            <Text intent=Intent::Danger>"Danger text"</Text>
                            <Text intent=Intent::Warning>"Warning text"</Text>
                            <Text intent=Intent::Info>"Info text"</Text>
                        </div>
                    </div>

                    // Font Weights
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Font Weights"
                        </Text>
                        <div class="space-y-2">
                            <Text weight=FontWeight::Normal>"Normal weight text"</Text>
                            <Text weight=FontWeight::Medium>"Medium weight text"</Text>
                            <Text weight=FontWeight::Semibold>"Semibold weight text"</Text>
                            <Text weight=FontWeight::Bold>"Bold weight text"</Text>
                        </div>
                    </div>
                </section>

                // Icon Section
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Icons"
                    </Text>
                    
                    // Icon Sizes
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Sizes"
                        </Text>
                        <div class="flex items-center gap-4">
                            <Icon size=Size::Xs name="home" />
                            <Icon size=Size::Sm name="home" />
                            <Icon size=Size::Md name="home" />
                            <Icon size=Size::Lg name="home" />
                            <Icon size=Size::Xl name="home" />
                        </div>
                    </div>

                    // Icon Variants
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Variants"
                        </Text>
                        <div class="flex items-center gap-4">
                            <div class="flex flex-col items-center gap-2">
                                <Icon variant=IconVariant::Outline name="check" />
                                <Text variant=TextVariant::Caption>"Outline"</Text>
                            </div>
                            <div class="flex flex-col items-center gap-2">
                                <Icon variant=IconVariant::Solid name="check" />
                                <Text variant=TextVariant::Caption>"Solid"</Text>
                            </div>
                        </div>
                    </div>

                    // Icon Intents
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Intents"
                        </Text>
                        <div class="flex items-center gap-4">
                            <Icon intent=Intent::Primary name="check" />
                            <Icon intent=Intent::Secondary name="check" />
                            <Icon intent=Intent::Success name="check" />
                            <Icon intent=Intent::Danger name="x" />
                            <Icon intent=Intent::Warning name="minus" />
                            <Icon intent=Intent::Info name="search" />
                        </div>
                    </div>

                    // Available Icons
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Available Icons"
                        </Text>
                        <div class="grid grid-cols-6 md:grid-cols-10 gap-4">
                            {["check", "x", "plus", "minus", "chevron-down", "chevron-up", 
                              "chevron-left", "chevron-right", "search", "menu", "home", "sun", "moon"]
                                .iter()
                                .map(|icon_name| view! {
                                    <div class="flex flex-col items-center gap-2 p-2">
                                        <Icon name=icon_name />
                                        <Text variant=TextVariant::Caption size=Size::Xs>
                                            {*icon_name}
                                        </Text>
                                    </div>
                                })
                                .collect_view()
                            }
                        </div>
                    </div>
                </section>

                // Card Section
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Cards"
                    </Text>
                    
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Card Variants"
                        </Text>
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                            <Card variant=CardVariant::Default>
                                <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Semibold>
                                    "Default Card"
                                </Text>
                                <Text variant=TextVariant::Body class="mt-2">
                                    "This is a default card with standard styling."
                                </Text>
                            </Card>
                            
                            <Card variant=CardVariant::Elevated>
                                <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Semibold>
                                    "Elevated Card"
                                </Text>
                                <Text variant=TextVariant::Body class="mt-2">
                                    "This card has enhanced shadow for elevation."
                                </Text>
                            </Card>
                            
                            <Card variant=CardVariant::Outlined>
                                <Text variant=TextVariant::Heading size=Size::Md weight=FontWeight::Semibold>
                                    "Outlined Card"
                                </Text>
                                <Text variant=TextVariant::Body class="mt-2">
                                    "This card uses a thicker border outline."
                                </Text>
                            </Card>
                        </div>
                    </div>
                </section>

                // Alert Section  
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Alerts"
                    </Text>
                    
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Alert Intents"
                        </Text>
                        <div class="space-y-3">
                            <Alert intent=Intent::Success>
                                "Success! Your action was completed successfully."
                            </Alert>
                            <Alert intent=Intent::Danger>
                                "Error! Something went wrong with your request."
                            </Alert>
                            <Alert intent=Intent::Warning>
                                "Warning! Please review your input before proceeding."
                            </Alert>
                            <Alert intent=Intent::Info>
                                "Info: Here's some helpful information for you."
                            </Alert>
                        </div>
                    </div>
                    
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Alert with Title"
                        </Text>
                        <Alert intent=Intent::Success title="Success">
                            "Your form has been submitted successfully and will be processed shortly."
                        </Alert>
                    </div>
                </section>

                // Select Section
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Select Dropdown"
                    </Text>
                    
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Basic Select"
                        </Text>
                        <div class="max-w-xs">
                            <Select 
                                options=vec![
                                    SelectOption::new("option1", "Option 1"),
                                    SelectOption::new("option2", "Option 2"), 
                                    SelectOption::new("option3", "Option 3"),
                                ]
                                placeholder="Choose an option..."
                            />
                        </div>
                    </div>
                    
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Select Intents"
                        </Text>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <Select 
                                intent=Intent::Success
                                options=vec![
                                    SelectOption::new("success1", "Success Option"),
                                ]
                                placeholder="Success select..."
                            />
                            <Select 
                                intent=Intent::Danger
                                options=vec![
                                    SelectOption::new("error1", "Error Option"),
                                ]
                                placeholder="Error select..."
                            />
                        </div>
                    </div>
                </section>

                // Spinner Section
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Spinners"
                    </Text>
                    
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Spinner Variants"
                        </Text>
                        <div class="flex items-center gap-8">
                            <div class="flex flex-col items-center gap-2">
                                <Spinner variant=SpinnerVariant::Circle />
                                <Text variant=TextVariant::Caption>"Circle"</Text>
                            </div>
                            <div class="flex flex-col items-center gap-2">
                                <Spinner variant=SpinnerVariant::Pulse />
                                <Text variant=TextVariant::Caption>"Pulse"</Text>
                            </div>
                            <div class="flex flex-col items-center gap-2">
                                <Spinner variant=SpinnerVariant::Dots />
                                <Text variant=TextVariant::Caption>"Dots"</Text>
                            </div>
                        </div>
                    </div>
                    
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Spinner Sizes"
                        </Text>
                        <div class="flex items-end gap-6">
                            <Spinner size=Size::Xs />
                            <Spinner size=Size::Sm />
                            <Spinner size=Size::Md />
                            <Spinner size=Size::Lg />
                            <Spinner size=Size::Xl />
                        </div>
                    </div>
                </section>

                // Molecules Section
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Molecules"
                    </Text>
                    
                    // Theme Switcher
                    <div class="space-y-4">
                        <Text variant=TextVariant::Label size=Size::Sm weight=FontWeight::Medium>
                            "Theme Switcher"
                        </Text>
                        <div class="p-6 border rounded-lg space-y-4">
                            <Text variant=TextVariant::Body>
                                "Toggle between light and dark themes. The switcher automatically updates all components."
                            </Text>
                            <div class="flex items-center gap-6 flex-wrap">
                                <div class="flex flex-col gap-2">
                                    <Text variant=TextVariant::Caption>"Compact"</Text>
                                    <ThemeSwitcher size=Size::Sm />
                                </div>
                                <div class="flex flex-col gap-2">
                                    <Text variant=TextVariant::Caption>"With Label"</Text>
                                    <ThemeSwitcher size=Size::Md show_label=true />
                                </div>
                                <div class="flex flex-col gap-2">
                                    <Text variant=TextVariant::Caption>"Large"</Text>
                                    <ThemeSwitcher size=Size::Lg show_label=true />
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Component Combinations
                <section class="space-y-6">
                    <Text variant=TextVariant::Heading size=Size::Lg weight=FontWeight::Semibold>
                        "Component Combinations"
                    </Text>
                    
                    <div class="space-y-6">
                        // Form Example
                        <div class="p-6 border rounded-lg space-y-4">
                            <Text variant=TextVariant::Heading size=Size::Md>
                                "Example Form"
                            </Text>
                            <div class="space-y-3">
                                <div>
                                    <Text variant=TextVariant::Label size=Size::Sm>
                                        "Name"
                                    </Text>
                                    <Input placeholder="Enter your name" />
                                </div>
                                <div>
                                    <Text variant=TextVariant::Label size=Size::Sm>
                                        "Email"
                                    </Text>
                                    <Input input_type=InputType::Email placeholder="Enter your email" />
                                </div>
                                <div class="flex gap-3">
                                    <Button intent=Intent::Primary>
                                        <Icon name="check" size=Size::Sm class="mr-2" />
                                        "Save"
                                    </Button>
                                    <Button intent=Intent::Secondary>
                                        <Icon name="x" size=Size::Sm class="mr-2" />
                                        "Cancel"
                                    </Button>
                                </div>
                            </div>
                        </div>

                        // Status Messages using Alert components
                        <Card variant=CardVariant::Default class="space-y-4">
                            <Text variant=TextVariant::Heading size=Size::Md>
                                "Status Messages"
                            </Text>
                            <div class="space-y-3">
                                <Alert intent=Intent::Success>
                                    <div class="flex items-center gap-2">
                                        <Icon name="check" intent=Intent::Success size=Size::Sm />
                                        "Operation completed successfully"
                                    </div>
                                </Alert>
                                <Alert intent=Intent::Danger>
                                    <div class="flex items-center gap-2">
                                        <Icon name="x" intent=Intent::Danger size=Size::Sm />
                                        "An error occurred"
                                    </div>
                                </Alert>
                                <Alert intent=Intent::Warning>
                                    <div class="flex items-center gap-2">
                                        <Icon name="minus" intent=Intent::Warning size=Size::Sm />
                                        "Please review your input"
                                    </div>
                                </Alert>
                                <Alert intent=Intent::Info>
                                    <div class="flex items-center gap-2">
                                        <Icon name="search" intent=Intent::Info size=Size::Sm />
                                        "Here's some helpful information"
                                    </div>
                                </Alert>
                            </div>
                        </Card>
                    </div>
                </section>
            </div>
    }
}