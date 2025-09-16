#![cfg(test)]

use {
    typefrog::{
        fact::IntoFacts,
        horn::{Clause, Prolog},
    },
    typeql::query::SchemaQuery,
};

#[test]
fn test_identifier_interning() {
    let input = r#"
    define
      entity content @abstract,
        owns id @key,
        owns is-visible,
        plays subscription:content;

      entity page @abstract, sub content,
        owns page-id,
        owns name,
        owns bio,
        owns profile-picture,
        owns badge,
        owns is-active,
        plays posting:page,
        plays viewing:viewed,
        plays following:page;

      entity profile @abstract, sub page,
        owns username,
        owns can-publish,
        plays group-membership:member,
        plays location:located,
        plays viewing:viewer,
        plays content-engagement:author,
        plays following:follower,
        plays subscription:subscriber;

      entity person sub profile,
        owns gender,
        owns language,
        owns email,
        owns phone,
        owns relationship-status,
        owns page-visibility,
        owns post-visibility,
        plays birth:born,
        plays friendship:friend,
        plays family:relative,
        plays parentship:parent,
        plays parentship:child,
        plays siblingship:sibling,
        plays relationship:partner,
        plays engagement:fiance,
        plays marriage:spouse,
        plays employment:employee,
        plays education:attendee;

      relation birth,
        relates born,
        owns birth-date,
        plays location:located;

      relation social-relation @abstract,
        relates related @card(2);

      relation friendship sub social-relation,
        relates friend as related @card(2);

      relation family sub social-relation,
        relates relative as related @card(2);

      relation parentship sub family,
        relates parent as relative,
        relates child as relative;

      relation siblingship sub family,
        relates sibling as relative @card(2);

      relation relationship sub social-relation,
        relates partner as related @card(2),
        owns start-date;

      relation engagement sub relationship,
        relates fiance as partner @card(2),
        owns engagement-date,
        plays location:located;

      relation marriage sub relationship,
        relates spouse as partner @card(2),
        owns marriage-date,
        plays location:located;

      entity organization sub profile,
        owns tag @card(0..),
        plays employment:employer;

      entity company sub organization;
      entity charity sub organization;

      entity educational-institute sub organization,
        plays education:institute;

      entity school sub educational-institute;
      entity college sub educational-institute;
      entity university sub educational-institute;

      relation employment,
        relates employer,
        relates employee,
        owns start-date,
        owns end-date,
        owns description;

      relation education,
        relates institute,
        relates attendee,
        owns start-date,
        owns end-date,
        owns description;

      entity group sub page,
        owns group-id,
        owns tag @card(0..),
        owns page-visibility,
        owns post-visibility,
        plays group-membership:group;

      relation group-membership,
        relates group,
        relates member,
        owns rank,
        owns badge,
        owns start-timestamp,
        owns end-timestamp;

      entity post @abstract, sub content,
        owns post-id,
        owns post-text,
        owns creation-timestamp,
        owns language,
        owns tag @card(0..),
        owns post-visibility,
        plays posting:post,
        plays sharing:original-post,
        plays commenting:parent,
        plays reaction:parent,
        plays location:located,
        plays viewing:viewed;

      entity text-post sub post;

      entity share-post sub post,
        plays sharing:share-post;

      entity image-post sub post,
        owns post-image;

      entity video-post sub post,
        owns post-video;

      entity live-video-post sub video-post;

      entity poll-post sub post,
        owns question,
        owns answer @card(0..),
        plays response:poll;

      entity comment sub content,
        owns comment-id,
        owns comment-text,
        owns creation-timestamp,
        owns tag @card(0..),
        plays commenting:comment,
        plays commenting:parent,
        plays reaction:parent;

      relation interaction @abstract,
        relates subject,
        relates content;

      relation viewing sub interaction,
        relates viewer as subject,
        relates viewed as content;

      relation content-engagement @abstract, sub interaction,
        relates author as subject;

      relation posting sub content-engagement,
        relates page as content,
        relates post;

      relation sharing sub content-engagement,
        relates original-post as content,
        relates share-post;

      relation commenting sub content-engagement,
        relates parent as content,
        relates comment;

      relation reaction sub content-engagement,
        relates parent as content,
        owns emoji,
        owns creation-timestamp;

      relation response sub content-engagement,
        relates poll as content,
        owns answer,
        owns creation-timestamp;

      relation following,
        relates follower,
        relates page;

      relation subscription,
        relates subscriber,
        relates content;

      entity place,
        owns place-id,
        owns name,
        plays location:place;

      entity region sub place,
        plays region-location:parent-region,
        plays region-location:child-region,
        plays country-location:region;

      entity country sub place,
        owns language @card(0..),
        plays country-location:country,
        plays state-location:country,
        plays city-location:parent,
        plays landmark-location:parent;

      entity state sub place,
        plays state-location:state,
        plays city-location:parent,
        plays landmark-location:parent;

      entity city sub place,
        plays city-location:city,
        plays landmark-location:parent;

      entity landmark sub place,
        owns lattitude,
        owns longitude,
        plays landmark-location:landmark;

      relation location,
        relates place,
        relates located;

      relation region-location sub location,
        relates parent-region as place,
        relates child-region as located;

      relation country-location sub location,
        relates region as place,
        relates country as located;

      relation state-location sub location,
        relates country as place,
        relates state as located;

      relation city-location sub location,
        relates parent as place,
        relates city as located;

      relation landmark-location sub location,
        relates parent as place,
        relates landmark as located;

      attribute id @abstract, value string;
      attribute page-id  @abstract, sub id;
      attribute username sub page-id;
      attribute group-id sub page-id;
      attribute post-id sub id;
      attribute comment-id sub id;
      attribute place-id sub id;

      attribute name, value string;
      attribute gender, value string @regex("^(male|female|other)$");
      attribute language, value string;
      attribute email, value string;
      attribute phone, value string;
      attribute relationship-status, value string @regex("^(single|relationship|engaged|married|complicated)$");
      attribute lattitude, value double;
      attribute longitude, value double;

      attribute event-date @abstract, value datetime;
      attribute start-date sub event-date;
      attribute end-date sub event-date;
      attribute exact-date @abstract, sub event-date;
      attribute birth-date sub exact-date;
      attribute engagement-date sub exact-date;
      attribute marriage-date sub exact-date;

      attribute payload @abstract, value string;
      attribute text-payload @abstract, sub payload;
      attribute media-payload @abstract, sub payload;
      attribute image-payload @abstract, sub media-payload;
      attribute video-payload @abstract, sub media-payload;
      attribute bio sub text-payload;
      attribute description sub text-payload;
      attribute profile-picture sub image-payload;
      attribute post-text sub text-payload;
      attribute post-image sub image-payload;
      attribute post-video sub video-payload;
      attribute question sub text-payload;
      attribute answer sub text-payload;
      attribute comment-text sub text-payload;

      attribute tag, value string;
      attribute rank, value string @regex("^(member|moderator|admin|owner)$");
      attribute emoji, value string @regex("^(like|love|funny|surprise|sad|angry)$");
      attribute badge, value string;

      attribute creation-timestamp, value datetime;
      attribute start-timestamp, value datetime;
      attribute end-timestamp, value datetime;

      attribute visibility @abstract, value string;
      attribute page-visibility sub visibility, value string @regex("^(public|private)$");
      attribute post-visibility sub visibility, value string @regex("^(default|public|private)$");

      attribute is-visible, value boolean;
      attribute is-active, value boolean;
      attribute can-publish, value boolean;

    # Transitively get all parent places of a place
    fun all_parent_places($place: place) -> { place }:
      match
        {
          location (located: $place, place: $parent);
        } or {
          location (located: $place, place: $middle);
          let $parent in all_parent_places($middle);
        };
      return { $parent };

    # Transitively get all child places of a place
    fun all_child_places($place: place) -> { place }:
        match
          $child isa place;
          {
            location (located: $child, place: $place);
          } or {
            location (located: $middle, place: $place);
            let $child in all_child_places($middle);
          };
      return { $child };

    # Transitively get child places of a place up to a given depth
    fun child_places($place: place, $depth: integer) -> { place }:
        match
          $child isa place;
          $depth > 0;
          {
            location (located: $child, place: $place);
          } or {
            location (located: $middle, place: $place);
            let $next_depth = $depth - 1;
            let $child in child_places($middle, $next_depth);
          };
      return { $child };

    fun parent_places_linked_list($place: place) -> { place, place }:
      match
        {
          location (located: $place, place: $parent);
          $child is $place;
        } or {
          location (located: $place, place: $middle);
          let $parent, $child in parent_places_linked_list($middle);
        };
      return { $parent, $child };

    fun located_in_transitive($place: place, $location: place) -> place:
      match
        {
          $location is $place;
          $parent is $place;
        } or {
          location (located: $place, place: $parent);
          let $_ = located_in_transitive($parent, $location);
        };
      return first $parent;"#;

    let results = match typeql::parse_queries(input) {
        Ok(queries) => queries
            .iter()
            .fold(vec![], |mut acc, query| match &query.structure {
                typeql::query::QueryStructure::Schema(SchemaQuery::Define(define)) => {
                    for definable in &define.definables {
                        match definable {
                            typeql::Definable::TypeDeclaration(type_) => {
                                acc.push(
                                    type_
                                        .into_facts()
                                        .into_iter()
                                        .map(|f| f.implication::<Prolog>())
                                        .collect::<Vec<String>>(),
                                );
                            }
                            typeql::Definable::Function(_function) => {}
                            typeql::Definable::Struct(_) => todo!(),
                        }
                    }
                    acc
                }
                _ => acc,
            }),
        Err(e) => {
            panic!("TypeQL parse error: {}", e);
        }
    };

    let combined = results.iter().flatten().fold(String::new(), |mut acc, s| {
        acc.push_str(s);
        acc
    });

    insta::assert_snapshot!(combined, @r#"
    entity(content). % 0
    abstract(content). % 0
    owns(content, id). % 0 1
    key(content). % 0
    owns(content, is-visible). % 0 2
    plays(content, subscription, content). % 0 3 0
    entity(page). % 4
    abstract(page). % 4
    sub(content, page). % 0 4
    owns(page, page-id). % 4 5
    owns(page, name). % 4 6
    owns(page, bio). % 4 7
    owns(page, profile-picture). % 4 8
    owns(page, badge). % 4 9
    owns(page, is-active). % 4 10
    plays(page, posting, page). % 4 11 4
    plays(page, viewing, viewed). % 4 12 13
    plays(page, following, page). % 4 14 4
    entity(profile). % 15
    abstract(profile). % 15
    sub(page, profile). % 4 15
    owns(profile, username). % 15 16
    owns(profile, can-publish). % 15 17
    plays(profile, group-membership, member). % 15 18 19
    plays(profile, location, located). % 15 20 21
    plays(profile, viewing, viewer). % 15 12 22
    plays(profile, content-engagement, author). % 15 23 24
    plays(profile, following, follower). % 15 14 25
    plays(profile, subscription, subscriber). % 15 3 26
    entity(person). % 27
    sub(profile, person). % 15 27
    owns(person, gender). % 27 28
    owns(person, language). % 27 29
    owns(person, email). % 27 30
    owns(person, phone). % 27 31
    owns(person, relationship-status). % 27 32
    owns(person, page-visibility). % 27 33
    owns(person, post-visibility). % 27 34
    plays(person, birth, born). % 27 35 36
    plays(person, friendship, friend). % 27 37 38
    plays(person, family, relative). % 27 39 40
    plays(person, parentship, parent). % 27 41 42
    plays(person, parentship, child). % 27 41 43
    plays(person, siblingship, sibling). % 27 44 45
    plays(person, relationship, partner). % 27 46 47
    plays(person, engagement, fiance). % 27 48 49
    plays(person, marriage, spouse). % 27 50 51
    plays(person, employment, employee). % 27 52 53
    plays(person, education, attendee). % 27 54 55
    rel(birth). % 35
    relates(birth, born). % 35 36
    owns(birth, birth-date). % 35 56
    plays(birth, location, located). % 35 20 21
    rel(social-relation). % 57
    abstract(social-relation). % 57
    relates(social-relation, related). % 57 58
    cardinality_exact(social-relation, 2). % 57 59
    rel(friendship). % 37
    sub(social-relation, friendship). % 57 37
    relates(friendship, friend). % 37 38
    cardinality_exact(friendship, 2). % 37 59
    rel(family). % 39
    sub(social-relation, family). % 57 39
    relates(family, relative). % 39 40
    cardinality_exact(family, 2). % 39 59
    rel(parentship). % 41
    sub(family, parentship). % 39 41
    relates(parentship, parent). % 41 42
    relates(parentship, child). % 41 43
    rel(siblingship). % 44
    sub(family, siblingship). % 39 44
    relates(siblingship, sibling). % 44 45
    cardinality_exact(siblingship, 2). % 44 59
    rel(relationship). % 46
    sub(social-relation, relationship). % 57 46
    relates(relationship, partner). % 46 47
    cardinality_exact(relationship, 2). % 46 59
    owns(relationship, start-date). % 46 60
    rel(engagement). % 48
    sub(relationship, engagement). % 46 48
    relates(engagement, fiance). % 48 49
    cardinality_exact(engagement, 2). % 48 59
    owns(engagement, engagement-date). % 48 61
    plays(engagement, location, located). % 48 20 21
    rel(marriage). % 50
    sub(relationship, marriage). % 46 50
    relates(marriage, spouse). % 50 51
    cardinality_exact(marriage, 2). % 50 59
    owns(marriage, marriage-date). % 50 62
    plays(marriage, location, located). % 50 20 21
    entity(organization). % 63
    sub(profile, organization). % 15 63
    owns(organization, tag). % 63 64
    cardinality_range(organization, 0, 0). % 63 65 65
    plays(organization, employment, employer). % 63 52 66
    entity(company). % 67
    sub(organization, company). % 63 67
    entity(charity). % 68
    sub(organization, charity). % 63 68
    entity(educational-institute). % 69
    sub(organization, educational-institute). % 63 69
    plays(educational-institute, education, institute). % 69 54 70
    entity(school). % 71
    sub(educational-institute, school). % 69 71
    entity(college). % 72
    sub(educational-institute, college). % 69 72
    entity(university). % 73
    sub(educational-institute, university). % 69 73
    rel(employment). % 52
    relates(employment, employer). % 52 66
    relates(employment, employee). % 52 53
    owns(employment, start-date). % 52 60
    owns(employment, end-date). % 52 74
    owns(employment, description). % 52 75
    rel(education). % 54
    relates(education, institute). % 54 70
    relates(education, attendee). % 54 55
    owns(education, start-date). % 54 60
    owns(education, end-date). % 54 74
    owns(education, description). % 54 75
    entity(group). % 76
    sub(page, group). % 4 76
    owns(group, group-id). % 76 77
    owns(group, tag). % 76 64
    cardinality_range(group, 0, 0). % 76 65 65
    owns(group, page-visibility). % 76 33
    owns(group, post-visibility). % 76 34
    plays(group, group-membership, group). % 76 18 76
    rel(group-membership). % 18
    relates(group-membership, group). % 18 76
    relates(group-membership, member). % 18 19
    owns(group-membership, rank). % 18 78
    owns(group-membership, badge). % 18 9
    owns(group-membership, start-timestamp). % 18 79
    owns(group-membership, end-timestamp). % 18 80
    entity(post). % 81
    abstract(post). % 81
    sub(content, post). % 0 81
    owns(post, post-id). % 81 82
    owns(post, post-text). % 81 83
    owns(post, creation-timestamp). % 81 84
    owns(post, language). % 81 29
    owns(post, tag). % 81 64
    cardinality_range(post, 0, 0). % 81 65 65
    owns(post, post-visibility). % 81 34
    plays(post, posting, post). % 81 11 81
    plays(post, sharing, original-post). % 81 85 86
    plays(post, commenting, parent). % 81 87 42
    plays(post, reaction, parent). % 81 88 42
    plays(post, location, located). % 81 20 21
    plays(post, viewing, viewed). % 81 12 13
    entity(text-post). % 89
    sub(post, text-post). % 81 89
    entity(share-post). % 90
    sub(post, share-post). % 81 90
    plays(share-post, sharing, share-post). % 90 85 90
    entity(image-post). % 91
    sub(post, image-post). % 81 91
    owns(image-post, post-image). % 91 92
    entity(video-post). % 93
    sub(post, video-post). % 81 93
    owns(video-post, post-video). % 93 94
    entity(live-video-post). % 95
    sub(video-post, live-video-post). % 93 95
    entity(poll-post). % 96
    sub(post, poll-post). % 81 96
    owns(poll-post, question). % 96 97
    owns(poll-post, answer). % 96 98
    cardinality_range(poll-post, 0, 0). % 96 65 65
    plays(poll-post, response, poll). % 96 99 100
    entity(comment). % 101
    sub(content, comment). % 0 101
    owns(comment, comment-id). % 101 102
    owns(comment, comment-text). % 101 103
    owns(comment, creation-timestamp). % 101 84
    owns(comment, tag). % 101 64
    cardinality_range(comment, 0, 0). % 101 65 65
    plays(comment, commenting, comment). % 101 87 101
    plays(comment, commenting, parent). % 101 87 42
    plays(comment, reaction, parent). % 101 88 42
    rel(interaction). % 104
    abstract(interaction). % 104
    relates(interaction, subject). % 104 105
    relates(interaction, content). % 104 0
    rel(viewing). % 12
    sub(interaction, viewing). % 104 12
    relates(viewing, viewer). % 12 22
    relates(viewing, viewed). % 12 13
    rel(content-engagement). % 23
    abstract(content-engagement). % 23
    sub(interaction, content-engagement). % 104 23
    relates(content-engagement, author). % 23 24
    rel(posting). % 11
    sub(content-engagement, posting). % 23 11
    relates(posting, page). % 11 4
    relates(posting, post). % 11 81
    rel(sharing). % 85
    sub(content-engagement, sharing). % 23 85
    relates(sharing, original-post). % 85 86
    relates(sharing, share-post). % 85 90
    rel(commenting). % 87
    sub(content-engagement, commenting). % 23 87
    relates(commenting, parent). % 87 42
    relates(commenting, comment). % 87 101
    rel(reaction). % 88
    sub(content-engagement, reaction). % 23 88
    relates(reaction, parent). % 88 42
    owns(reaction, emoji). % 88 106
    owns(reaction, creation-timestamp). % 88 84
    rel(response). % 99
    sub(content-engagement, response). % 23 99
    relates(response, poll). % 99 100
    owns(response, answer). % 99 98
    owns(response, creation-timestamp). % 99 84
    rel(following). % 14
    relates(following, follower). % 14 25
    relates(following, page). % 14 4
    rel(subscription). % 3
    relates(subscription, subscriber). % 3 26
    relates(subscription, content). % 3 0
    entity(place). % 107
    owns(place, place-id). % 107 108
    owns(place, name). % 107 6
    plays(place, location, place). % 107 20 107
    entity(region). % 109
    sub(place, region). % 107 109
    plays(region, region-location, parent-region). % 109 110 111
    plays(region, region-location, child-region). % 109 110 112
    plays(region, country-location, region). % 109 113 109
    entity(country). % 114
    sub(place, country). % 107 114
    owns(country, language). % 114 29
    cardinality_range(country, 0, 0). % 114 65 65
    plays(country, country-location, country). % 114 113 114
    plays(country, state-location, country). % 114 115 114
    plays(country, city-location, parent). % 114 116 42
    plays(country, landmark-location, parent). % 114 117 42
    entity(state). % 118
    sub(place, state). % 107 118
    plays(state, state-location, state). % 118 115 118
    plays(state, city-location, parent). % 118 116 42
    plays(state, landmark-location, parent). % 118 117 42
    entity(city). % 119
    sub(place, city). % 107 119
    plays(city, city-location, city). % 119 116 119
    plays(city, landmark-location, parent). % 119 117 42
    entity(landmark). % 120
    sub(place, landmark). % 107 120
    owns(landmark, lattitude). % 120 121
    owns(landmark, longitude). % 120 122
    plays(landmark, landmark-location, landmark). % 120 117 120
    rel(location). % 20
    relates(location, place). % 20 107
    relates(location, located). % 20 21
    rel(region-location). % 110
    sub(location, region-location). % 20 110
    relates(region-location, parent-region). % 110 111
    relates(region-location, child-region). % 110 112
    rel(country-location). % 113
    sub(location, country-location). % 20 113
    relates(country-location, region). % 113 109
    relates(country-location, country). % 113 114
    rel(state-location). % 115
    sub(location, state-location). % 20 115
    relates(state-location, country). % 115 114
    relates(state-location, state). % 115 118
    rel(city-location). % 116
    sub(location, city-location). % 20 116
    relates(city-location, parent). % 116 42
    relates(city-location, city). % 116 119
    rel(landmark-location). % 117
    sub(location, landmark-location). % 20 117
    relates(landmark-location, parent). % 117 42
    relates(landmark-location, landmark). % 117 120
    attribute(id). % 1
    abstract(id). % 1
    value(id, String). % 1 123
    attribute(page-id). % 5
    abstract(page-id). % 5
    sub(id, page-id). % 1 5
    attribute(username). % 16
    sub(page-id, username). % 5 16
    attribute(group-id). % 77
    sub(page-id, group-id). % 5 77
    attribute(post-id). % 82
    sub(id, post-id). % 1 82
    attribute(comment-id). % 102
    sub(id, comment-id). % 1 102
    attribute(place-id). % 108
    sub(id, place-id). % 1 108
    attribute(name). % 6
    value(name, String). % 6 123
    attribute(gender). % 28
    value(gender, String). % 28 123
    regex(gender, "^(male|female|other)$"). % 28 124
    attribute(language). % 29
    value(language, String). % 29 123
    attribute(email). % 30
    value(email, String). % 30 123
    attribute(phone). % 31
    value(phone, String). % 31 123
    attribute(relationship-status). % 32
    value(relationship-status, String). % 32 123
    regex(relationship-status, "^(single|relationship|engaged|married|complicated)$"). % 32 125
    attribute(lattitude). % 121
    value(lattitude, f64). % 121 126
    attribute(longitude). % 122
    value(longitude, f64). % 122 126
    attribute(event-date). % 127
    abstract(event-date). % 127
    value(event-date, DateTime). % 127 128
    attribute(start-date). % 60
    sub(event-date, start-date). % 127 60
    attribute(end-date). % 74
    sub(event-date, end-date). % 127 74
    attribute(exact-date). % 129
    abstract(exact-date). % 129
    sub(event-date, exact-date). % 127 129
    attribute(birth-date). % 56
    sub(exact-date, birth-date). % 129 56
    attribute(engagement-date). % 61
    sub(exact-date, engagement-date). % 129 61
    attribute(marriage-date). % 62
    sub(exact-date, marriage-date). % 129 62
    attribute(payload). % 130
    abstract(payload). % 130
    value(payload, String). % 130 123
    attribute(text-payload). % 131
    abstract(text-payload). % 131
    sub(payload, text-payload). % 130 131
    attribute(media-payload). % 132
    abstract(media-payload). % 132
    sub(payload, media-payload). % 130 132
    attribute(image-payload). % 133
    abstract(image-payload). % 133
    sub(media-payload, image-payload). % 132 133
    attribute(video-payload). % 134
    abstract(video-payload). % 134
    sub(media-payload, video-payload). % 132 134
    attribute(bio). % 7
    sub(text-payload, bio). % 131 7
    attribute(description). % 75
    sub(text-payload, description). % 131 75
    attribute(profile-picture). % 8
    sub(image-payload, profile-picture). % 133 8
    attribute(post-text). % 83
    sub(text-payload, post-text). % 131 83
    attribute(post-image). % 92
    sub(image-payload, post-image). % 133 92
    attribute(post-video). % 94
    sub(video-payload, post-video). % 134 94
    attribute(question). % 97
    sub(text-payload, question). % 131 97
    attribute(answer). % 98
    sub(text-payload, answer). % 131 98
    attribute(comment-text). % 103
    sub(text-payload, comment-text). % 131 103
    attribute(tag). % 64
    value(tag, String). % 64 123
    attribute(rank). % 78
    value(rank, String). % 78 123
    regex(rank, "^(member|moderator|admin|owner)$"). % 78 135
    attribute(emoji). % 106
    value(emoji, String). % 106 123
    regex(emoji, "^(like|love|funny|surprise|sad|angry)$"). % 106 136
    attribute(badge). % 9
    value(badge, String). % 9 123
    attribute(creation-timestamp). % 84
    value(creation-timestamp, DateTime). % 84 128
    attribute(start-timestamp). % 79
    value(start-timestamp, DateTime). % 79 128
    attribute(end-timestamp). % 80
    value(end-timestamp, DateTime). % 80 128
    attribute(visibility). % 137
    abstract(visibility). % 137
    value(visibility, String). % 137 123
    attribute(page-visibility). % 33
    sub(visibility, page-visibility). % 137 33
    value(page-visibility, String). % 33 123
    regex(page-visibility, "^(public|private)$"). % 33 138
    attribute(post-visibility). % 34
    sub(visibility, post-visibility). % 137 34
    value(post-visibility, String). % 34 123
    regex(post-visibility, "^(default|public|private)$"). % 34 139
    attribute(is-visible). % 2
    value(is-visible, bool). % 2 140
    attribute(is-active). % 10
    value(is-active, bool). % 10 140
    attribute(can-publish). % 17
    value(can-publish, bool). % 17 140
    "#);

    insta::assert_debug_snapshot!(typefrog::compute(input).unwrap(), @r#"
    IR {
        structs: [
            Struct {
                name: "charity",
                fields: [
                    Field {
                        name: "badge",
                        ty: "String",
                    },
                    Field {
                        name: "can-publish",
                        ty: "bool",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-active",
                        ty: "bool",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "profile",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "can-publish",
                                return_type: "bool",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "page",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "city",
                fields: [
                    Field {
                        name: "name",
                        ty: "String",
                    },
                ],
                impls: [],
            },
            Struct {
                name: "college",
                fields: [
                    Field {
                        name: "badge",
                        ty: "String",
                    },
                    Field {
                        name: "can-publish",
                        ty: "bool",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-active",
                        ty: "bool",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "profile",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "can-publish",
                                return_type: "bool",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "page",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "comment",
                fields: [
                    Field {
                        name: "creation-timestamp",
                        ty: "DateTime",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "company",
                fields: [
                    Field {
                        name: "badge",
                        ty: "String",
                    },
                    Field {
                        name: "can-publish",
                        ty: "bool",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-active",
                        ty: "bool",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "profile",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "can-publish",
                                return_type: "bool",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "page",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "country",
                fields: [
                    Field {
                        name: "language",
                        ty: "String",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                ],
                impls: [],
            },
            Struct {
                name: "educational-institute",
                fields: [
                    Field {
                        name: "badge",
                        ty: "String",
                    },
                    Field {
                        name: "can-publish",
                        ty: "bool",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-active",
                        ty: "bool",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "profile",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "can-publish",
                                return_type: "bool",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "page",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "group",
                fields: [
                    Field {
                        name: "badge",
                        ty: "String",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-active",
                        ty: "bool",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                    Field {
                        name: "page-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "post-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "page",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "image-post",
                fields: [
                    Field {
                        name: "creation-timestamp",
                        ty: "DateTime",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "language",
                        ty: "String",
                    },
                    Field {
                        name: "post-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "post",
                        funcs: [
                            Fn {
                                name: "creation-timestamp",
                                return_type: "DateTime",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "language",
                                return_type: "String",
                            },
                            Fn {
                                name: "post-visibility",
                                return_type: "String",
                            },
                            Fn {
                                name: "tag",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "landmark",
                fields: [
                    Field {
                        name: "lattitude",
                        ty: "f64",
                    },
                    Field {
                        name: "longitude",
                        ty: "f64",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                ],
                impls: [],
            },
            Struct {
                name: "live-video-post",
                fields: [
                    Field {
                        name: "creation-timestamp",
                        ty: "DateTime",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "language",
                        ty: "String",
                    },
                    Field {
                        name: "post-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "post",
                        funcs: [
                            Fn {
                                name: "creation-timestamp",
                                return_type: "DateTime",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "language",
                                return_type: "String",
                            },
                            Fn {
                                name: "post-visibility",
                                return_type: "String",
                            },
                            Fn {
                                name: "tag",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "organization",
                fields: [
                    Field {
                        name: "badge",
                        ty: "String",
                    },
                    Field {
                        name: "can-publish",
                        ty: "bool",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-active",
                        ty: "bool",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "profile",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "can-publish",
                                return_type: "bool",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "page",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "person",
                fields: [
                    Field {
                        name: "badge",
                        ty: "String",
                    },
                    Field {
                        name: "can-publish",
                        ty: "bool",
                    },
                    Field {
                        name: "email",
                        ty: "String",
                    },
                    Field {
                        name: "gender",
                        ty: "String",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-active",
                        ty: "bool",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "language",
                        ty: "String",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                    Field {
                        name: "page-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "phone",
                        ty: "String",
                    },
                    Field {
                        name: "post-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "relationship-status",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "profile",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "can-publish",
                                return_type: "bool",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "page",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "place",
                fields: [
                    Field {
                        name: "name",
                        ty: "String",
                    },
                ],
                impls: [],
            },
            Struct {
                name: "poll-post",
                fields: [
                    Field {
                        name: "creation-timestamp",
                        ty: "DateTime",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "language",
                        ty: "String",
                    },
                    Field {
                        name: "post-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "post",
                        funcs: [
                            Fn {
                                name: "creation-timestamp",
                                return_type: "DateTime",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "language",
                                return_type: "String",
                            },
                            Fn {
                                name: "post-visibility",
                                return_type: "String",
                            },
                            Fn {
                                name: "tag",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "region",
                fields: [
                    Field {
                        name: "name",
                        ty: "String",
                    },
                ],
                impls: [],
            },
            Struct {
                name: "school",
                fields: [
                    Field {
                        name: "badge",
                        ty: "String",
                    },
                    Field {
                        name: "can-publish",
                        ty: "bool",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-active",
                        ty: "bool",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "profile",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "can-publish",
                                return_type: "bool",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "page",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "share-post",
                fields: [
                    Field {
                        name: "creation-timestamp",
                        ty: "DateTime",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "language",
                        ty: "String",
                    },
                    Field {
                        name: "post-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "post",
                        funcs: [
                            Fn {
                                name: "creation-timestamp",
                                return_type: "DateTime",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "language",
                                return_type: "String",
                            },
                            Fn {
                                name: "post-visibility",
                                return_type: "String",
                            },
                            Fn {
                                name: "tag",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "state",
                fields: [
                    Field {
                        name: "name",
                        ty: "String",
                    },
                ],
                impls: [],
            },
            Struct {
                name: "text-post",
                fields: [
                    Field {
                        name: "creation-timestamp",
                        ty: "DateTime",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "language",
                        ty: "String",
                    },
                    Field {
                        name: "post-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "post",
                        funcs: [
                            Fn {
                                name: "creation-timestamp",
                                return_type: "DateTime",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "language",
                                return_type: "String",
                            },
                            Fn {
                                name: "post-visibility",
                                return_type: "String",
                            },
                            Fn {
                                name: "tag",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "university",
                fields: [
                    Field {
                        name: "badge",
                        ty: "String",
                    },
                    Field {
                        name: "can-publish",
                        ty: "bool",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-active",
                        ty: "bool",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "name",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "profile",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "can-publish",
                                return_type: "bool",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "page",
                        funcs: [
                            Fn {
                                name: "badge",
                                return_type: "String",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-active",
                                return_type: "bool",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "name",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
            Struct {
                name: "video-post",
                fields: [
                    Field {
                        name: "creation-timestamp",
                        ty: "DateTime",
                    },
                    Field {
                        name: "id",
                        ty: "String",
                    },
                    Field {
                        name: "is-visible",
                        ty: "bool",
                    },
                    Field {
                        name: "language",
                        ty: "String",
                    },
                    Field {
                        name: "post-visibility",
                        ty: "String",
                    },
                    Field {
                        name: "tag",
                        ty: "String",
                    },
                ],
                impls: [
                    Trait {
                        name: "post",
                        funcs: [
                            Fn {
                                name: "creation-timestamp",
                                return_type: "DateTime",
                            },
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                            Fn {
                                name: "language",
                                return_type: "String",
                            },
                            Fn {
                                name: "post-visibility",
                                return_type: "String",
                            },
                            Fn {
                                name: "tag",
                                return_type: "String",
                            },
                        ],
                    },
                    Trait {
                        name: "content",
                        funcs: [
                            Fn {
                                name: "id",
                                return_type: "String",
                            },
                            Fn {
                                name: "is-visible",
                                return_type: "bool",
                            },
                        ],
                    },
                ],
            },
        ],
        traits: [
            Trait {
                name: "content",
                funcs: [
                    Fn {
                        name: "id",
                        return_type: "String",
                    },
                    Fn {
                        name: "is-visible",
                        return_type: "bool",
                    },
                ],
            },
            Trait {
                name: "page",
                funcs: [
                    Fn {
                        name: "badge",
                        return_type: "String",
                    },
                    Fn {
                        name: "id",
                        return_type: "String",
                    },
                    Fn {
                        name: "is-active",
                        return_type: "bool",
                    },
                    Fn {
                        name: "is-visible",
                        return_type: "bool",
                    },
                    Fn {
                        name: "name",
                        return_type: "String",
                    },
                ],
            },
            Trait {
                name: "post",
                funcs: [
                    Fn {
                        name: "creation-timestamp",
                        return_type: "DateTime",
                    },
                    Fn {
                        name: "id",
                        return_type: "String",
                    },
                    Fn {
                        name: "is-visible",
                        return_type: "bool",
                    },
                    Fn {
                        name: "language",
                        return_type: "String",
                    },
                    Fn {
                        name: "post-visibility",
                        return_type: "String",
                    },
                    Fn {
                        name: "tag",
                        return_type: "String",
                    },
                ],
            },
            Trait {
                name: "profile",
                funcs: [
                    Fn {
                        name: "badge",
                        return_type: "String",
                    },
                    Fn {
                        name: "can-publish",
                        return_type: "bool",
                    },
                    Fn {
                        name: "id",
                        return_type: "String",
                    },
                    Fn {
                        name: "is-active",
                        return_type: "bool",
                    },
                    Fn {
                        name: "is-visible",
                        return_type: "bool",
                    },
                    Fn {
                        name: "name",
                        return_type: "String",
                    },
                ],
            },
        ],
        relations: [],
    }
    "#);
}
