use crate::{
    common::model::BangumiClient,
    module::{character::model::CharacterSearch, model::SimpleImageType},
};

#[tokio::test]
async fn test_search_characters() {
    let client = BangumiClient::default();
    let request = CharacterSearch {
        keyword: "夏目".to_string(),
        filter: None,
    };
    let result = client.search_characters(None, None, Some(request)).await;
    assert!(result.is_ok(), "搜索角色失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_character() {
    let client = BangumiClient::default();
    let result = client.get_character(5).await;
    assert!(result.is_ok(), "获取角色详情失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_character_image() {
    let client = BangumiClient::default();
    let result = client.get_character_image(5, SimpleImageType::Large).await;
    assert!(result.is_ok(), "获取角色图片失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_character_subjects() {
    let client = BangumiClient::default();
    let result = client.get_character_subjects(5).await;
    assert!(result.is_ok(), "获取关联条目失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_character_persons() {
    let client = BangumiClient::default();
    let result = client.get_character_persons(5).await;
    assert!(result.is_ok(), "获取关联人物失败: {:?}", result.err());
}

// 注意: collect和uncollect测试需要认证，这里只测试API调用是否成功
// 实际使用时需要先登录
#[tokio::test]
async fn test_collect_character() {
    let client = BangumiClient::default();
    let collect_result = client.collect_character(5).await;
    assert!(
        collect_result.is_ok(),
        "收藏角色失败: {:?}",
        collect_result.err()
    );
}

// 实际使用时需要先登录
#[tokio::test]
async fn test_uncollect_character() {
    let client = BangumiClient::default();
    let uncollect_result = client.uncollect_character(5).await;
    assert!(
        uncollect_result.is_ok(),
        "取消收藏角色失败: {:?}",
        uncollect_result.err()
    );
}
