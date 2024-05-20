use crate::release::{ChangelogCommit, ChangelogFooter};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

impl Serialize for ChangelogCommit<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut commit = serializer.serialize_struct("Commit", 10)?;

        let footers = &self
            .commit
            .conventional
            .footers
            .iter()
            .map(ChangelogFooter::from)
            .collect::<Vec<ChangelogFooter>>();

        commit.serialize_field("id", &self.commit.oid)?;
        commit.serialize_field("author", &self.author_username)?;
        commit.serialize_field("signature", &self.commit.author)?;
        commit.serialize_field("type", &self.changelog_title)?;
        commit.serialize_field("date", &self.commit.date)?;
        commit.serialize_field("scope", &self.commit.conventional.scope)?;
        commit.serialize_field("summary", &self.commit.conventional.summary)?;
        commit.serialize_field("body", &self.commit.conventional.body)?;
        commit.serialize_field(
            "breaking_change",
            &self.commit.conventional.is_breaking_change,
        )?;
        commit.serialize_field("footer", footers)?;
        commit.end()
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use cocogitto_config::SETTINGS;
    use git2::Oid;
    use speculoos::prelude::*;

    use crate::release::ChangelogCommit;
    use cocogitto_commit::{Commit, CommitType, ConventionalCommit, Footer};
    use cocogitto_tag::Tag;

    #[test]
    fn should_serialize_tag() {
        let oid = Oid::from_str("1234567890").unwrap();
        let tag = Tag::from_str(
            "1.0.0",
            Some(oid),
            None,
            SETTINGS.tag_prefix(),
            SETTINGS.monorepo_separator(),
            SETTINGS.package_names(),
        )
        .unwrap();

        let result = toml::to_string(&tag);

        assert_that!(result)
            .is_ok()
            .is_equal_to("\"1.0.0\"".to_string())
    }

    #[test]
    fn should_serialize_commit() {
        let commit = ChangelogCommit {
            changelog_title: "BugFix".to_string(),
            author_username: Some("Jm Doudou"),
            commit: Commit {
                oid: "1234567890".to_string(),
                conventional: ConventionalCommit {
                    commit_type: CommitType::BugFix,
                    scope: Some("parser".to_string()),
                    summary: "fix parser implementation".to_string(),
                    body: Some("the body".to_string()),
                    footers: vec![Footer {
                        token: "token".to_string(),
                        content: "content".to_string(),
                        ..Default::default()
                    }],
                    is_breaking_change: false,
                },
                author: "Jean Michel Doudou".to_string(),
                date: Utc::now().naive_utc(),
            },
        };

        let result = toml::to_string(&commit);

        assert_that!(result).is_ok();
    }
}