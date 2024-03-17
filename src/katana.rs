use regex::{Captures, Regex};

use lazy_static::lazy_static;

fn remove_composite_abbreviations(text: &str) -> String {
    Regex::new(r"(?P<comp>et al\.)(?:\.)")
        .unwrap()
        .replace_all(text, "$comp&;&")
        .to_string()
}

fn remove_suspension_points(text: &str) -> String {
    Regex::new(r"\.{3}")
        .unwrap()
        .replace_all(text, "&&&.")
        .to_string()
}

fn remove_floating_point_numbers(text: &str) -> String {
    Regex::new(r"(?P<number>[0-9]+)\.(?P<decimal>[0-9]+)")
        .unwrap()
        .replace_all(text, "$number&@&$decimal")
        .to_string()
}

fn handle_floats_without_leading_zero(text: &str) -> String {
    Regex::new(r"\s\.(?P<nums>[0-9]+)")
        .unwrap()
        .replace_all(text, " &#&$nums")
        .to_string()
}

fn remove_abbreviations(text: &str) -> String {
    Regex::new(r"(?:[A-Za-z]\.){2,}")
        .unwrap()
        .replace_all(text, |caps: &regex::Captures| {
            caps.iter()
                .filter_map(|c| c.map(|c| c.as_str().to_string().replace(".", "&-&")))
                .collect::<String>()
        })
        .to_string()
}

fn remove_initials(text: &str) -> String {
    Regex::new(r"(?P<init>[A-Z])(?P<point>\.)")
        .unwrap()
        .replace_all(text, "$init&_&")
        .to_string()
}

fn remove_titles(text: &str) -> String {
    Regex::new(r"(?P<title>[A-Z][a-z]{1,3})(\.)")
        .unwrap()
        .replace_all(text, "$title&*&")
        .to_string()
}

fn unstick_sentences(text: &str) -> String {
    Regex::new(r##"(?P<left>[^.?!]\.|!|\?)(?P<right>[^\s"'])"##)
        .unwrap()
        .replace_all(text, "$left $right")
        .to_string()
}

fn remove_sentence_enders_before_parens(text: &str) -> String {
    Regex::new(r##"(?P<bef>[.?!])\s?\)"##)
        .unwrap()
        .replace_all(text, "&==&$bef")
        .to_string()
}

fn remove_sentence_enders_next_to_quotes(text: &str) -> String {
    let transformations = [
        (r##"'(?P<quote>[.?!])\s?""##, "&^&$quote"),
        (r##"'(?P<quote>[.?!])\s?”"##, "&**&$quote"),
        (r##"(?P<quote>[.?!])\s?”"##, "&=&$quote"),
        (r##"(?P<quote>[.?!])\s?'""##, "&,&$quote"),
        (r##"(?P<quote>[.?!])\s?'"##, "&##&$quote"),
        (r##"(?P<quote>[.?!])\s?""##, "&$quote"),
    ];
    transformations
        .iter()
        .fold(text.to_string(), |acc, (pattern, repl)| {
            Regex::new(pattern)
                .unwrap()
                .replace_all(&acc, *repl)
                .to_string()
        })
}

fn split_sentences(text: &str) -> Vec<Vec<String>> {
    let mut paragraphs: Vec<Vec<String>> = Vec::new();
    let mut current_sentence = String::new();
    let mut current_paragraph = Vec::new();

    for c in text.chars() {
        if c == '\n' {
            if !current_sentence.is_empty() {
                current_paragraph.push(current_sentence.clone());
                current_sentence.clear();
            }
            if !current_paragraph.is_empty() {
                paragraphs.push(current_paragraph.clone());
                current_paragraph.clear();
            }
        } else {
            current_sentence.push(c);
            if c == '.' || c == '?' || c == '!' {
                current_paragraph.push(current_sentence.clone());
                current_sentence.clear();
            }
        }
    }

    if !current_sentence.is_empty() {
        current_paragraph.push(current_sentence);
    }
    if !current_paragraph.is_empty() {
        paragraphs.push(current_paragraph);
    }

    paragraphs
}

fn repair_sentences(paragraphs: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let paren_repair = Regex::new(r"&==&(?P<p>[.!?])").unwrap();
    let quote_repair_regexes = [
        Regex::new(r"&\^&(?P<p>[.!?])").unwrap(),
        Regex::new(r"&\*\*&(?P<p>[.!?])").unwrap(),
        Regex::new(r"&=&(?P<p>[.!?])").unwrap(),
        Regex::new(r#"&,&(?P<p>[.!?])"#).unwrap(),
        Regex::new(r"&##&(?P<p>[.!?])").unwrap(),
        Regex::new(r"&\$&(?P<p>[.!?])").unwrap(),
    ];

    let repaired_paragraphs = paragraphs.into_iter().map(|paragraph| {
        paragraph.into_iter().map(|s| {
            let replaced_sentence = s.trim()
                .replace("&;&", ".")
                .replace("&&&", "..")
                .replace("&@&", ".")
                .replace("&#&", ".")
                .replace("&-&", ".")
                .replace("&_&", ".")
                .replace("&*&", ".");
            let paren_repaired = paren_repair.replace_all(&replaced_sentence, r"$1)").to_string();
            quote_repair_regexes.iter().fold(paren_repaired, |acc, regex| {
                regex.replace_all(
                    &acc,
                    match regex as *const Regex {
                        x if x == &quote_repair_regexes[0] as *const Regex => r#"'$p""#,
                        x if x == &quote_repair_regexes[1] as *const Regex => r#"'$p”"#,
                        x if x == &quote_repair_regexes[2] as *const Regex => r#"$p”"#,
                        x if x == &quote_repair_regexes[3] as *const Regex => r#"$p""#,
                        x if x == &quote_repair_regexes[4] as *const Regex => r#"$p'"#,
                        _ => r#"$p""#,
                    },
                ).to_string()
            })
        }).filter(|s| !s.is_empty()).collect()
    }).filter(|p: &Vec<String>| !p.is_empty()).collect();

    repaired_paragraphs
}

pub fn cut(origin_text: &String) -> Vec<Vec<String>> {
    let mut text = remove_composite_abbreviations(origin_text);
    text = remove_suspension_points(&text);
    text = remove_floating_point_numbers(&text);
    text = handle_floats_without_leading_zero(&text);
    text = remove_abbreviations(&text);
    text = remove_initials(&text);
    text = remove_titles(&text);
    text = unstick_sentences(&text);
    text = remove_sentence_enders_before_parens(&text);
    text = remove_sentence_enders_next_to_quotes(&text);
    let paragraphs = split_sentences(&text);
    repair_sentences(paragraphs)
}

#[cfg(test)]
mod test {
    extern crate test;

    use self::test::Bencher;

    #[test]
    fn it_works() {
        let text = String::from("For years, people in the U.A.E.R. have accepted murky air, tainted waters and scarred landscapes as the unavoidable price of the country’s meteoric economic growth. But public dissent over environmental issues has been growing steadily in the communist nation, and now seems to be building the foundations of a fledgling green movement! In July alone, two separate demonstrations made international news when they turned violent after about 1.5 minutes... These recent successes come after a slew of ever-larger and more violent green protests over the past few years, as the environmentalist Dr. C. Jeung of China’s growth becomes harder to ignore.Some ask: “Are demonstrations are evidence of the public anger and frustration at opaque environmental management and decision-making?” Others yet say: \"Should we be scared about these 'protests'?\" The man made a quick calculation and found the result to be .625. (This is another sentence in parens.) This is the last sentence.");
        let result = vec![
            "For years, people in the U.A.E.R. have accepted murky air, tainted waters and scarred landscapes as the unavoidable price of the country’s meteoric economic growth.",
            "But public dissent over environmental issues has been growing steadily in the communist nation, and now seems to be building the foundations of a fledgling green movement!",
            "In July alone, two separate demonstrations made international news when they turned violent after about 1.5 minutes...",
            "These recent successes come after a slew of ever-larger and more violent green protests over the past few years, as the environmentalist Dr. C. Jeung of China’s growth becomes harder to ignore.",
            "Some ask: “Are demonstrations are evidence of the public anger and frustration at opaque environmental management and decision-making?”",
            "Others yet say: \"Should we be scared about these 'protests'?\"",
            "The man made a quick calculation and found the result to be .625.",
            "(This is another sentence in parens.)",
            "This is the last sentence.",
        ];
        assert_eq!(result, super::cut(&text));
    }

    #[bench]
    fn bench_cut_short(b: &mut Bencher) {
        let short_text = String::from("Text one. Text two.");
        b.iter(|| super::cut(&short_text));
    }
    #[bench]
    fn bench_cut_long(b: &mut Bencher) {
        let long_text = String::from(
            r##"Sweetgreen restaurant in Bethesda, Md., on Oct. 28, 2010. (Jeffrey MacMillan) If you’re worried you packed on the pounds from all those holiday treats and feasts there’s a new style of restaurant beckoning you to the lighter side. Chains like Native Foods, Sweetgreen, Laughing Planet and Lyfe Kitchen are making a splash by dishing out plant-based meals quickly, holding prices down and emphasizing ethical eating with animal welfare and environmental sustainability. Proponents have dubbed them “healthy fast food” and dream they may solve the crises of obesity, factory farming, and global warming one value meal at a time. If it sounds too good to be true, it is. Not one healthy food chain can match fast food’s speed and convenience, nor can they compete on cost. Even worse, healthy fast food can cast a “health halo” over consumers, leading them to unwittingly order meals as unhealthy as their fast-food counterparts. The root problem is whole foods are ill-suited for fast food. Fresh vegetables and most fruit are delivery vehicles for nutrients, fiber and water. They are low in calories, fat and protein, so you have to eat a lot to fill up. Active men, for example, should eat nine servings of fruit and vegetables daily. As such, fresh foods are more expensive, laborious and wasteful to ship, store and prepare than slapping frozen burgers on the grill. The resource-intensive nature of vegetables, and hence expense, was also indicated by the recent Carnegie-Mellon study that found per calorie, lettuce generated three times as much greenhouse gas emissions as bacon did. It’s why we need to eat a lot of produce to fill up. Forget the five-a-day rule. An active 20-year-old male, for example, should eat 13 servings of fruit and vegetables a day, according to the Centers for Disease Control. Those factors, and taxpayer subsidies, keep fast food cheap and fresh foods pricey. At McDonald’s, the average check is $4.75. At Lyfe Kitchen it’s “around $15.” Native Foods and Veggie Grill are probably in the same range as nearly all their entrees are north of $9. The salads at Sweetgreen, which is so trendy these days it merited a profile in the New Yorker, are even pricier at $9.50 to almost $14 with tax. The difference is starker based on cost per calorie, with Sweetgreen’s salads five to 13 times more expensive than a McDouble from the dollar menu. Fast food is also fast to make, buy and eat. Drive-thru orders account for 57 percent of visits at burger chains. So important is this segment to the industry that KFC spent two years engineering a “Go Cup” and customized chicken and potato products to fill it. Fresh salads and bowls take time to assemble, and you have to sit down to eat them. The higher cost and lack of convenience at healthy chains means few fast-food stalwarts are likely switching over. Okay, so healthy food isn’t cheap and it isn’t quick. At least it’s good for you, right? Yes and no. Healthy chains do use more vegetables, fruit, beans and whole grains, which are woefully underrepresented on our plates. But to keep us coming back for more, they rely on the junk-food trinity of fat, salt and sugar. Adults should get 20 percent to 35 percent of their calories from fat. But at Tender Greens, Chopt, Laughing Planet and Sweetgreen, the menus bristle with fat bombs that are more than 50 percent fat by calories. At Lyfe Kitchen, which a co-founder calls a “healthy, inviting, sustainable McDonald’s” that features “very little” fat on the menu, only one salad is less than 70 percent fat. Lyfe’s widely praised Brussels sprouts, which it sees as “an alternative to french fries,” are 53 percent fat. Lyfe is no outlier, as it’s commonplace for restaurants to flavor vegetables with tasty stuff like cheese, bacon and olive oil. A little-noticed USDA study in 2014 found that eating more vegetables resulted in consuming more calories and sodium overall, particularly at restaurants. The worst offenders were potatoes and tomatoes, which is the majority of our vegetable intake. We all know the line about “pizza is a complete meal” because it has grain, vegetables, dairy and meat. Turns out the joke’s on us. When we eat a cup of tomatoes out of home we wolf down an extra 364 calories; at home it’s only 59 additional calories. This is where the health halo comes into play. If you believe a fast-food outlet is healthy then you’re prone to consume more calories than at a “bad” one like Burger King. Consider how healthy chains slather fat on kale, the nutritional superstar. Lyfe Kitchen’s kale salad has more fat than a Big Mac. At Laughing Planet, the large “Highway to Kale” has more fat than a bacon-and-cheese Whopper. And the 65 grams of fat in a Savory Kale Caesar from Veggie Grill is equal to the amount of fat most adults should consume in a day. When we go out, eating healthy is a low priority. Despite spending more money on dining out than on groceries, Americans get only 10 percent of their vegetables and 2 percent of fresh fruit at restaurants, and fast food in particular is a drag on fresh produce consumption. Expecting the fast-food sector to help solve the obesity crisis is like asking bars to promote sobriety. Two vegetarian chains shared a list of top sellers with The Washington Post. Veggie Grill’s favorites include Santa Fe Crispy Chickin’, Crispy Cauliflower, Buffalo Wings, Bombay Bowl and Quinoa Power Salad, all high in fat and sodium. In June, Amy’s Drive Thru opened the first of a planned chain of organic vegetarian eateries, and calls its food “clean” and “better-for-you” (a phrase also used by Frito-Lay), rather than healthy. But its marketing sprinkles in salubrious buzzwords like organic, non-GMO, fresh veggies and vegan. Apart from a couple of salads, its top sellers are burgers, burritos, pizza, chili fries, mac-n-cheese and milkshakes. Many consumers also consider Chipotle healthy because of its “Food with integrity” campaign, but the average order there is 1,070 calories -- more than half the daily allowance for most adults. Ironically, if you don’t treat healthy chains like fast food, they can be healthier than traditional burger joints. That means reading the nutritional information carefully, skipping salty fatty sauces and forgoing any fries, chips, sweet drinks or desserts. But if you really want to eat healthy, experts say, there’s really no place like home. Read more: This diet study upends everything we thought we knew about ‘healthy’ food Cutting sugar from kids’ diets appears to have a beneficial effect in just 10 days Scientists (sort of) settle debate on low-carb vs. low-fat diets Doritos, deconstructed (mesmerizing photos of the 34 processed ingredients in your favorite snack) Hot topic: Could regularly eating spicy foods help you live longer? Beware the rule-following co-worker, Harvard study warns For more health news, you can sign up for our weekly newsletter here."##,
        );
        b.iter(|| super::cut(&long_text));
    }
}
