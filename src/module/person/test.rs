use crate::{
    common::model::BangumiClient,
    module::{model::SimpleImageType, person::model::PersonSearch},
};

#[tokio::test]
async fn test_search_persons() {
    let client = BangumiClient::default();
    // 测试搜索人物
    let request = PersonSearch {
        keyword: "神谷".to_string(),
        filter: None,
    };
    let result = client.search_persons(None, None, Some(request)).await;
    assert!(result.is_ok(), "搜索人物失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_person_detail() {
    let client = BangumiClient::default();
    let result = client.get_person(6).await;
    assert!(result.is_ok(), "获取人物详情失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_person_image() {
    let client = BangumiClient::default();
    let result = client.get_person_image(6, SimpleImageType::Large).await;
    assert!(result.is_ok(), "获取人物图片失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_person_subjects() {
    let client = BangumiClient::default();
    let result = client.get_person_subjects(6).await;
    assert!(result.is_ok(), "获取关联条目失败: {:?}", result.err());
}

#[tokio::test]
async fn test_get_person_characters() {
    let client = BangumiClient::default();
    let result = client.get_person_characters(1).await;
    assert!(result.is_ok(), "获取关联角色失败: {:?}", result.err());
}

// 以下测试需要认证，仅验证API调用是否成功
#[tokio::test]
async fn test_collect_person() {
    let client = BangumiClient::default();
    let result = client.collect_person(6).await;
    assert!(result.is_ok(), "收藏人物失败: {:?}", result.err());
}

#[tokio::test]
async fn test_uncollect_person() {
    let client = BangumiClient::default();
    let result = client.uncollect_person(6).await;
    assert!(result.is_ok(), "取消收藏人物失败: {:?}", result.err());
}
